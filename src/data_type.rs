use std::{
    ffi::{CStr, c_void},
    fmt::Debug,
    mem::{self, MaybeUninit},
    ptr,
};

use open62541_sys::{
    UA_DataType, UA_Order, UA_STATUSCODE_GOOD, UA_clear, UA_copy, UA_init, UA_new, UA_order,
    UA_print,
};

use crate::ua;

/// Transparent wrapper for OPC UA data type.
///
/// # Safety
///
/// It must be possible to transmute between the type that implements [`DataType`] and the inner
/// type [`Inner`]. This implies that `#[repr(transparent)]` must be used on types that implement
/// this trait and the inner type must be [`Inner`].
///
/// In addition, the inner type must not contain self-references (in Rust terms, it would have to
/// implement the [`Unpin`] trait). This is usually the case for types from [`open62541_sys`], as
/// they are regularly passed by value to functions in order to transfer ownership.
///
/// [`Inner`]: DataType::Inner
pub unsafe trait DataType: Debug + Clone {
    /// Inner type.
    ///
    /// It must be possible to transmute between the inner type and the type that implements
    /// [`DataType`].
    type Inner;

    /// Gets `open62541` data type record.
    ///
    /// The result can be passed to functions in `open62541` that deal with arbitrary data types.
    #[must_use]
    fn data_type() -> *const UA_DataType;

    /// Gets data type name.
    #[must_use]
    fn type_name() -> &'static str {
        let data_type = Self::data_type();
        // SAFETY: `data_type` is a valid pointer.
        unsafe { CStr::from_ptr((*data_type).typeName) }
            .to_str()
            // PANIC: `typeName` is an ASCII string.
            .expect("string should be valid")
    }

    /// Creates wrapper by taking ownership of value.
    ///
    /// When `Self` is dropped, [`UA_clear()`] is used to free allocations held by the inner type.
    /// Move only values into `Self` that can be cleared in-place such as stack-allocated values
    /// (but no heap-allocated values created by [`UA_new()`]).
    ///
    /// # Safety
    ///
    /// Ownership of the value passes to `Self`. This must only be used for values that are not
    /// contained within other values that may be dropped (such as attributes in other data types).
    /// In this case use [`clone_raw()`] or [`take_raw()`] instead to clone or move data instead of
    /// taking ownership.
    ///
    /// [`UA_new()`]: open62541_sys::UA_new
    /// [`clone_raw()`]: DataType::clone_raw
    /// [`take_raw()`]: DataType::take_raw
    #[must_use]
    unsafe fn from_raw(src: Self::Inner) -> Self;

    /// Gives up ownership and returns value.
    ///
    /// The returned value must be re-wrapped with [`from_raw()`] or cleared manually with
    /// [`UA_clear()`] to free internal allocations and not leak memory.
    ///
    /// [`from_raw()`]: DataType::from_raw
    #[must_use]
    fn into_raw(self) -> Self::Inner;

    /// Creates wrapper initialized with defaults.
    ///
    /// This uses [`UA_init()`] to initialize the value and make all attributes well-defined.
    /// Depending on the type, additional attributes may need to be initialized for the value to be
    /// actually useful afterwards.
    #[must_use]
    fn init() -> Self {
        let mut inner = MaybeUninit::<Self::Inner>::uninit();
        // `UA_init()` may depend on the specific data type. The current implementation just
        // zero-initializes the memory region. We could use `MaybeUninit::zeroed()` instead but we
        // want to allow `open62541` to use a different implementation in the future if necessary.
        unsafe {
            UA_init(inner.as_mut_ptr().cast::<c_void>(), Self::data_type());
        }
        // SAFETY: We just made sure that the memory region is initialized.
        let inner = unsafe { inner.assume_init() };
        // SAFETY: We pass a value without pointers to it into `Self`.
        unsafe { Self::from_raw(inner) }
    }

    /// Creates wrapper reference from value.
    #[must_use]
    fn raw_ref(src: &Self::Inner) -> &Self {
        let src: *const Self::Inner = src;
        // This transmutes between the inner type and `Self` through `cast()`. Types that implement
        // `DataType` guarantee that we can transmute between them and their inner type, so this is
        // okay.
        let ptr = src.cast::<Self>();
        // SAFETY: `DataType` guarantees that we can transmute between `Self` and the inner type.
        let ptr = unsafe { ptr.as_ref() };
        // SAFETY: Pointer is valid (non-zero) because it comes from a reference.
        unsafe { ptr.unwrap_unchecked() }
    }

    /// Creates mutable wrapper reference from value.
    #[must_use]
    fn raw_mut(src: &mut Self::Inner) -> &mut Self {
        let src: *mut Self::Inner = src;
        // This transmutes between the inner type and `Self` through `cast()`. Types that implement
        // `DataType` guarantee that we can transmute between them and their inner type, so this is
        // okay.
        let ptr = src.cast::<Self>();
        // SAFETY: `DataType` guarantees that we can transmute between `Self` and the inner type.
        let ptr = unsafe { ptr.as_mut() };
        // SAFETY: Pointer is valid (non-zero) because it comes from a reference.
        unsafe { ptr.unwrap_unchecked() }
    }

    /// Creates wrapper by cloning value from `src`.
    ///
    /// This uses [`UA_copy()`] to deeply copy an existing value without transferring ownership.
    ///
    /// The original value must be cleared with [`UA_clear()`], or deleted with [`UA_delete()`] if
    /// allocated on the heap, to avoid memory leaks. If `src` is borrowed from another data type
    /// wrapper, that wrapper will make sure of this.
    ///
    /// [`UA_delete()`]: open62541_sys::UA_delete
    #[must_use]
    fn clone_raw(src: &Self::Inner) -> Self {
        let src: *const Self::Inner = src;
        // `UA_copy()` does not clean up the target value before copying into it, so we may use an
        // uninitialized memory region here.
        let mut dst = MaybeUninit::<Self::Inner>::uninit();
        let result = unsafe {
            UA_copy(
                src.cast::<c_void>(),
                dst.as_mut_ptr().cast::<c_void>(),
                Self::data_type(),
            )
        };
        assert_eq!(result, UA_STATUSCODE_GOOD, "should have copied value");
        // SAFETY: We just made sure that the memory region is initialized.
        let dst = unsafe { dst.assume_init() };
        // SAFETY: We pass a value without pointers to it into `Self`.
        unsafe { Self::from_raw(dst) }
    }

    /// Creates wrapper by moving value from `src`.
    ///
    /// This moves an existing value and uses [`UA_init()`] to leave an initialized value behind.
    #[must_use]
    fn take_raw(src: &mut Self::Inner) -> Self {
        // Take out source value and leave behind a freshly initialized value that gets cleaned up
        // when the external owner frees memory.
        let src = mem::replace(src, Self::init().into_raw());
        // SAFETY: We have replaced the original source with a freshly initialized value. There is
        // no other owner and we take ownership.
        unsafe { Self::from_raw(src) }
    }

    // /// Creates copy without giving up ownership.
    // ///
    // /// # Safety
    // ///
    // /// Think twice before using this. Pointers to dynamically allocated attributes within `src` are
    // /// copied and will become dangling pointers in `src` when the _returned_ value is dropped. Vice
    // /// versa, directly (or indirectly) freeing the copied value later would double-free memory when
    // /// the returned value is dropped.
    // #[must_use]
    // unsafe fn copy_raw(src: &Self::Inner) -> Self {
    //     todo!()
    // }

    /// Clones value into `dst`.
    ///
    /// This uses [`UA_copy()`] to deeply copy an existing value without transferring ownership.
    ///
    /// Existing data in `dst` is cleared with [`UA_clear()`] before cloning the value; it is safe
    /// to use this operation on already initialized target values.
    fn clone_into_raw(&self, dst: &mut Self::Inner) {
        let dst: *mut Self::Inner = dst;
        // `UA_copy()` does not clean up the target value before copying into it, so we use
        // `UA_clear()` first to free dynamically allocated memory held by the current value.
        unsafe {
            UA_clear(dst.cast::<c_void>(), Self::data_type());
        }
        // Copy ourselves into the target. This duplicates and allocates memory if necessary to
        // store a copy of the inner value.
        let result = unsafe {
            UA_copy(
                self.as_ptr().cast::<c_void>(),
                dst.cast::<c_void>(),
                Self::data_type(),
            )
        };
        assert_eq!(result, UA_STATUSCODE_GOOD, "should have copied value");
    }

    /// Moves value into `dst`, giving up ownership.
    ///
    /// Existing data in `dst` is cleared with [`UA_clear()`] before moving the value; it is safe
    /// to use this operation on already initialized target values.
    ///
    /// After this, it is the responsibility of `dst` to eventually clean up the data.
    fn move_into_raw(self, dst: &mut Self::Inner) {
        let dst: *mut Self::Inner = dst;
        // Use `UA_clear()` first to free dynamically allocated memory held by the current value.
        unsafe {
            UA_clear(dst.cast::<c_void>(), Self::data_type());
        }
        // Move ourselves into the target. This keeps existing memory allocations but we do not
        // reference them anymore because `into_raw()` gives up ownership.
        unsafe { *dst = self.into_raw() };
    }

    /// Leaks wrapped value onto the heap.
    ///
    /// This turns a stack-allocated value into a heap-allocated one, without issuing a deep copy.
    /// In other words, only the local memory allocation is copied (moved from stack to heap) and
    /// any memory that is already heap-allocated (e.g. string contents) stays where it is.
    ///
    /// The returned value must be passed into another owned data structure or freed manually with
    /// [`UA_delete()`] to free internal allocations and not leak memory.
    ///
    /// [`UA_delete()`]: open62541_sys::UA_delete
    #[must_use]
    fn leak_into_raw(self) -> *mut Self::Inner {
        // Use `UA_new()` to create heap allocation that can be cleaned up with `UA_free()`.
        let dst = unsafe { UA_new(Self::data_type()) }.cast::<Self::Inner>();
        // Check that heap allocation was successful (we might be out of memory).
        assert!(!dst.is_null(), "should have allocated heap memory");
        // SAFETY: Pointer is valid (non-zero) because we just checked it.
        self.move_into_raw(unsafe { dst.as_mut().unwrap_unchecked() });
        dst
    }

    /// Creates copy without giving up ownership.
    ///
    /// # Safety
    ///
    /// Think twice before using this. Pointers to dynamically allocated attributes within `Self`
    /// are copied and will become dangling pointers when `this` is dropped. Vice versa, directly
    /// (or indirectly) freeing the copied value would double-free memory when `this` is dropped.
    ///
    /// This function is necessary because some functions in `open62541` take arguments by value
    /// instead of by pointer _without taking ownership_ and make the caller responsible for
    /// cleaning up after the call has returned.
    #[must_use]
    unsafe fn to_raw_copy(this: &Self) -> Self::Inner {
        // SAFETY: This creates a copy of the inner value despite it not being `Copy`. Extreme care
        // must be taken that any contained structures are not freed twice when `self` is dropped.
        unsafe { ptr::read(this.as_ptr()) }
    }

    /// Returns shared reference to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[must_use]
    unsafe fn as_ref(&self) -> &Self::Inner {
        // SAFETY: We wrap the pointer into a reference below to ensure upholding lifetime rules.
        let ptr = unsafe { self.as_ptr() };
        // SAFETY: `DataType` guarantees that we can transmute between `Self` and the inner type.
        let ptr = unsafe { ptr.as_ref() };
        // SAFETY: Pointer is valid (non-zero) because it comes from a reference.
        unsafe { ptr.unwrap_unchecked() }
    }

    /// Returns exclusive reference to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[must_use]
    unsafe fn as_mut(&mut self) -> &mut Self::Inner {
        // SAFETY: We wrap the pointer into a reference below to ensure upholding lifetime rules.
        let ptr = unsafe { self.as_mut_ptr() };
        // SAFETY: `DataType` guarantees that we can transmute between `Self` and the inner type.
        let ptr = unsafe { ptr.as_mut() };
        // SAFETY: Pointer is valid (non-zero) because it comes from a reference.
        unsafe { ptr.unwrap_unchecked() }
    }

    /// Returns const pointer to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[must_use]
    unsafe fn as_ptr(&self) -> *const Self::Inner {
        let this: *const Self = self;
        // This transmutes between `Self` and the inner type through `cast()`. Types that implement
        // `DataType` guarantee that we can transmute between them and their inner type, so this is
        // okay.
        this.cast::<Self::Inner>()
    }

    /// Returns mutable pointer to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[must_use]
    unsafe fn as_mut_ptr(&mut self) -> *mut Self::Inner {
        let this: *mut Self = self;
        // This transmutes between `Self` and the inner type through `cast()`. Types that implement
        // `DataType` guarantee that we can transmute between them and their inner type, so this is
        // okay.
        this.cast::<Self::Inner>()
    }

    /// Prints value to string.
    ///
    /// This uses [`UA_print()`] to generate the string representation.
    ///
    /// # Note
    ///
    /// The string representation is not guaranteed to be stable across versions.
    #[must_use]
    fn print(this: &Self) -> Option<ua::String> {
        let mut output = ua::String::init();
        let result = unsafe {
            UA_print(
                this.as_ptr().cast::<c_void>(),
                Self::data_type(),
                <ua::String as DataType>::as_mut_ptr(&mut output),
            )
        };
        (result == UA_STATUSCODE_GOOD).then_some(output)
    }

    /// Compares value to other.
    ///
    /// This uses [`UA_order()`] to derive a total ordering between values.
    #[must_use]
    fn order(this: &Self, other: &Self) -> UA_Order {
        unsafe {
            UA_order(
                this.as_ptr().cast::<std::ffi::c_void>(),
                other.as_ptr().cast::<std::ffi::c_void>(),
                Self::data_type(),
            )
        }
    }
}

/// Defines wrapper for OPC UA data type from [`open62541_sys`].
///
/// This provides the basic interface to convert from and back into the [`open62541_sys`] types. Use
/// another `impl` block to add additional methods to each type if necessary.
macro_rules! data_type {
    ($name:ident) => {
        paste::paste! {
            $crate::data_type!($name, [<UA_ $name>], [<UA_TYPES_ $name:upper>]);
        }
    };

    ($name:ident, $inner:ident) => {
        paste::paste! {
            $crate::data_type!($name, [<UA_ $name>], [<UA_TYPES_ $inner:upper>]);
        }
    };

    ($name:ident, $inner:ident, $index:ident) => {
        /// Wrapper for
        #[doc = concat!("[`", stringify!($inner), "`](open62541_sys::", stringify!($inner), ")")]
        /// from [`open62541_sys`].
        ///
        /// This owns the wrapped data. When the wrapper is dropped, the inner value is cleaned up
        /// with [`UA_clear()`] to release dynamically allocated memory held by the value.
        ///
        /// [`UA_clear()`]: open62541_sys::UA_clear
        #[repr(transparent)]
        pub struct $name(
            /// Inner value.
            open62541_sys::$inner,
        );

        // SAFETY: The types in `open62541` can be sent across thread boundaries. They contain
        // pointers but all internal dynamic allocations contain only their own data (nothing is
        // shared) and the allocations need not be deallocated in the same thread where they were
        // allocated.
        unsafe impl Send for $name {}

        // SAFETY: References to [`DataType`] may be sent across threads. The inner types would not
        // allow this (because pointers are used to pass ownership) but we must unwrap our wrapper
        // types in this case which is only implemented for owned values.
        unsafe impl Sync for $name {}

        impl Drop for $name {
            fn drop(&mut self) {
                // `UA_clear()` resets the data structure, freeing any dynamically allocated memory
                // in it, no matter how deeply nested.
                unsafe {
                    open62541_sys::UA_clear(
                        (&raw mut self.0).cast::<std::ffi::c_void>(),
                        <Self as $crate::DataType>::data_type(),
                    )
                }
            }
        }

        // SAFETY: We can transmute between our wrapper type and the inner type. This is ensured by
        // using `#[repr(transparent)]` on the type definition.
        unsafe impl $crate::DataType for $name {
            type Inner = open62541_sys::$inner;

            fn data_type() -> *const open62541_sys::UA_DataType {
                // PANIC: Value must fit into `usize` to allow indexing.
                let index = usize::try_from(open62541_sys::$index).unwrap();
                let ua_types = &raw const open62541_sys::UA_TYPES;
                // SAFETY: Pointer is non-zero, aligned, correct type.
                // PANIC: The given index is valid within `UA_TYPES`.
                unsafe { (*ua_types).get(index) }.unwrap()
            }

            unsafe fn from_raw(src: Self::Inner) -> Self {
                $name(src)
            }

            fn into_raw(self) -> Self::Inner {
                unsafe { std::mem::transmute(self) }
            }
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                <Self as $crate::DataType>::clone_raw(&self.0)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let output = <Self as $crate::DataType>::print(self);
                let string = output.as_ref().and_then(|output| output.as_str());
                // Do not apply any formatting flags to the stringified value.
                f.write_str(string.unwrap_or(stringify!($name)))
            }
        }

        impl std::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                <Self as std::cmp::Ord>::cmp(self, other) == std::cmp::Ordering::Equal
            }
        }

        // The implementation of [`UA_order()`] ensures an equivalence relation. Among others, the
        // comparison of floating point numbers deviates from IEEE 754 and handles NaN as proper
        // values.
        impl std::cmp::Eq for $name {}

        impl std::cmp::PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(<Self as std::cmp::Ord>::cmp(self, other))
            }
        }

        // The implementation of [`UA_order()`] ensures a total order.
        impl std::cmp::Ord for $name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                let result = <Self as $crate::DataType>::order(self, other);
                match result {
                    open62541_sys::UA_Order::UA_ORDER_LESS => std::cmp::Ordering::Less,
                    open62541_sys::UA_Order::UA_ORDER_EQ => std::cmp::Ordering::Equal,
                    open62541_sys::UA_Order::UA_ORDER_MORE => std::cmp::Ordering::Greater,
                    _ => panic!("should return valid order"),
                }
            }
        }
    };
}

pub(crate) use data_type;

/// Defines known enum variants for wrapper.
///
/// This allows implementing data types that wrap an enum type from [`open62541_sys`]. This provides
/// `const` members for each given variant and implements [`Display`]. Use this with [`data_type!`].
///
/// [`Display`]: std::fmt::Display
macro_rules! enum_variants {
    ($name:ident, $inner:ident, [$( $value:ident ),* $(,)?] $(,)?) => {
        impl $name {
            $(
                /// Enum variant
                #[doc = paste::paste! { concat!("[`", stringify!([<$inner:upper _ $value>]), "`](open62541_sys::", stringify!($inner), "::", stringify!([<$inner:upper _ $value>]), ")") }]
                /// from [`open62541_sys`].
                #[expect(clippy::allow_attributes, reason = "not required for all variants")]
                #[allow(dead_code, reason = "unused `pub`-declared constants in private modules")]
                pub const $value: Self = Self(
                    paste::paste! { open62541_sys::$inner::[<$inner:upper _ $value>] }
                );

                paste::paste! {
                    // This cast is necessary on Windows builds with inner type `i32`.
                    #[expect(clippy::allow_attributes, reason = "dynamic condition")]
                    #[allow(clippy::as_conversions, trivial_numeric_casts, reason = "bindgen i32")]
                    pub const [<$value _U32>]: u32 = open62541_sys::$inner::[<$inner:upper _ $value>].0 as u32;
                }
            )*

            #[expect(clippy::allow_attributes, reason = "not required for all variants")]
            #[allow(dead_code, reason = "not used for all variants")]
            pub(crate) fn from_u32(value: u32) -> Self {
                // This cast is necessary on Windows builds with inner type `i32`.
                Self(open62541_sys::$inner(value.try_into().expect("should convert from u32")))
            }

            pub(crate) fn as_u32(&self) -> u32 {
                // This cast is necessary on Windows builds with inner type `i32`.
                #[expect(clippy::allow_attributes, reason = "dynamic condition")]
                #[allow(clippy::useless_conversion, reason = "bindgen i32")]
                u32::try_from((self.0).0).expect("should convert to u32")
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let str = match self.0 {
                    $(
                        paste::paste! { open62541_sys::$inner::[<$inner:upper _ $value>] } => {
                            stringify!($value)
                        },
                    )*

                    _ => return self.as_u32().fmt(f),
                };

                str.fmt(f)
            }
        }
    };
}

pub(crate) use enum_variants;

macro_rules! bitmask_ops {
    ($name:ident) => {
        impl $name {
            /// Gets logical OR of two masks.
            #[must_use]
            pub const fn or(&self, other: &Self) -> Self {
                Self::from_u32(self.as_u32() | other.as_u32())
            }
        }

        impl std::ops::BitOr for $name {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self {
                self.or(&rhs)
            }
        }
    };
}

pub(crate) use bitmask_ops;

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn send_sync_string() {
        let string = ua::String::new("Lorem Ipsum").expect("create string");

        // References to string can be accessed in different threads.
        thread::scope(|scope| {
            scope.spawn(|| {
                let _ = &string;
            });
        });

        // Ownership of string can be passed to different thread.
        thread::spawn(move || {
            drop(string);
        })
        .join()
        .expect("join thread");
    }
}
