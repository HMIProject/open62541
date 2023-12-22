use std::{ffi::c_void, mem::MaybeUninit};

use open62541_sys::{UA_copy, UA_init, UA_print, UA_STATUSCODE_GOOD};

use crate::ua;

/// Transparent wrapper for OPC UA data type.
///
/// # Safety
///
/// It must be possible to transmute between the type that implements [`DataType`] and the inner
/// type [`Inner`]. This implies that `#[repr(transparent)]` must be used on types that implement
/// this trait and the inner type must be [`Inner`].
///
/// [`Inner`]: DataType::Inner
pub unsafe trait DataType: Clone {
    /// Inner type.
    ///
    /// It must be possible to transmute between the inner type and the type that implements
    /// [`DataType`].
    type Inner;

    /// Gets `open62541` data type record.
    ///
    /// The result can be passed to functions in `open62541` that deal with arbitrary data types.
    #[must_use]
    fn data_type() -> *const open62541_sys::UA_DataType;

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
    /// In this case use [`clone_raw()`] instead to clone data instead of taking ownership.
    ///
    /// [`UA_clear()`]: open62541_sys::UA_clear
    /// [`UA_new()`]: open62541_sys::UA_new
    /// [`clone_raw()`]: DataType::clone_raw
    #[must_use]
    unsafe fn from_raw(src: Self::Inner) -> Self;

    /// Gives up ownership and returns inner value.
    ///
    /// The returned value must be re-wrapped with [`from_raw()`] or cleared manually with
    /// [`UA_clear()`] to free internal allocations and not leak memory.
    ///
    /// [`from_raw()`]: DataType::from_raw
    /// [`UA_clear()`]: open62541_sys::UA_clear
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
            UA_init(
                inner.as_mut_ptr().cast::<std::ffi::c_void>(),
                <Self as crate::DataType>::data_type(),
            );
        }
        // SAFETY: We just made sure that the memory region is initialized.
        let inner = unsafe { inner.assume_init() };
        // SAFETY: We pass a value without pointers to it into `Self`.
        unsafe { Self::from_raw(inner) }
    }

    /// Creates wrapper by cloning value from `src`.
    ///
    /// The original value must be cleared with [`UA_clear()`], or deleted with [`UA_delete()`] if
    /// allocated on the heap, to avoid memory leaks. If `src` is borrowed from another data type
    /// wrapper, that wrapper will make sure of this.
    ///
    /// [`UA_clear()`]: open62541_sys::UA_clear
    /// [`UA_delete()`]: open62541_sys::UA_delete
    #[must_use]
    fn clone_raw(src: &Self::Inner) -> Self {
        // `UA_copy()` does not clean up the target value before copying into it, so we may use an
        // uninitialized memory region here.
        let mut dst = MaybeUninit::<Self::Inner>::uninit();
        let result = unsafe {
            UA_copy(
                (src as *const Self::Inner).cast::<std::ffi::c_void>(),
                dst.as_mut_ptr().cast::<std::ffi::c_void>(),
                <Self as crate::DataType>::data_type(),
            )
        };
        assert_eq!(result, open62541_sys::UA_STATUSCODE_GOOD);
        // SAFETY: We just made sure that the memory region is initialized.
        let dst = unsafe { dst.assume_init() };
        // SAFETY: We pass a value without pointers to it into `Self`.
        unsafe { Self::from_raw(dst) }
    }

    // TODO
    // #[must_use]
    // fn get_ref(src: &Self::Inner) -> &Self {
    //     // This transmutes between the inner type and `Self` through `cast()`. Types that implement
    //     // `DataType` guarantee that we can transmute between them and their inner type, so this is
    //     // okay.
    //     let ptr = (src as *const Self::Inner).cast::<Self>();
    //     // SAFETY: `DataType` guarantees that we can transmute between `Self` and the inner type.
    //     let ptr = unsafe { ptr.as_ref() };
    //     // SAFETY: Pointer is valid (non-zero) because it comes from a reference.
    //     unsafe { ptr.unwrap_unchecked() }
    // }

    /// Returns shared reference to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[must_use]
    unsafe fn as_ref(&self) -> &Self::Inner {
        let ptr = self.as_ptr();
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
        let ptr = self.as_mut_ptr();
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
        // This transmutes between `Self` and the inner type through `cast()`. Types that implement
        // `DataType` guarantee that we can transmute between them and their inner type, so this is
        // okay.
        (self as *const Self).cast::<Self::Inner>()
    }

    /// Returns mutable pointer to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[must_use]
    unsafe fn as_mut_ptr(&mut self) -> *mut Self::Inner {
        // This transmutes between `Self` and the inner type through `cast()`. Types that implement
        // `DataType` guarantee that we can transmute between them and their inner type, so this is
        // okay.
        (self as *mut Self).cast::<Self::Inner>()
    }

    #[must_use]
    fn print(&self) -> Option<ua::String> {
        let mut output = ua::String::init();
        let result = unsafe {
            UA_print(
                self.as_ptr().cast::<c_void>(),
                Self::data_type(),
                output.as_mut_ptr(),
            )
        };
        (result == UA_STATUSCODE_GOOD).then_some(output)
    }
}

/// Define wrapper for OPC UA data type from [`open62541_sys`].
///
/// This provides the basic interface to convert from and back into the [`open62541_sys`] types. Use
/// another `impl` block to add additional methods to each type if necessary.
macro_rules! data_type {
    ($name:ident, $inner:ident, $index:ident) => {
        /// Wrapper for
        #[doc = concat!("[`", stringify!($inner), "`](open62541_sys::", stringify!($inner), ")")]
        /// from [`open62541_sys`].
        ///
        /// This owns the wrapped data type. When the wrapper is dropped, its inner value, including
        /// all contained data, is cleaned up with [`UA_clear()`](open62541_sys::UA_clear()).
        #[repr(transparent)]
        pub struct $name(
            /// Inner value.
            open62541_sys::$inner,
        );

        impl $name {
            /// Clones inner value into target.
            ///
            /// This makes sure to clean up any existing value in `dst` before cloning the value. It
            /// is therefore safe to use on already initialized target values. The original value in
            /// the target is overwritten.
            #[allow(dead_code)]
            pub(crate) fn clone_into(&self, dst: &mut open62541_sys::$inner) {
                // Clear the target and free any dynamically allocated memory there from the current
                // value.
                unsafe {
                    open62541_sys::UA_clear(
                        std::ptr::addr_of_mut!(*dst).cast::<std::ffi::c_void>(),
                        <Self as crate::DataType>::data_type(),
                    )
                }

                // Copy ourselves into the target. This duplicates and allocates memory if necessary
                // to store a copy of the inner value.
                let result = unsafe {
                    open62541_sys::UA_copy(
                        std::ptr::addr_of!(self.0).cast::<std::ffi::c_void>(),
                        std::ptr::addr_of_mut!(*dst).cast::<std::ffi::c_void>(),
                        <Self as crate::DataType>::data_type(),
                    )
                };
                assert_eq!(result, open62541_sys::UA_STATUSCODE_GOOD);
            }
        }

        // SAFETY: The types in `open62541` can be sent across thread boundaries. Internally, all of
        // the internal dynamic allocations contain only their own data (nothing is shared) and they
        // need not be freed in the same thread where they were allocated.
        unsafe impl Send for $name {}

        // SAFETY: References to our wrapper types may be sent across thread. (The `open62541` types
        // themselves would not allow this because references are used to pass ownership but we must
        // unwrap our wrapper types in this case which we do not implement for shared references.)
        unsafe impl Sync for $name {}

        impl Drop for $name {
            fn drop(&mut self) {
                // `UA_clear()` resets the data structure, freeing any dynamically allocated memory
                // in it, no matter how deeply nested.
                unsafe {
                    open62541_sys::UA_clear(
                        std::ptr::addr_of_mut!(self.0).cast::<std::ffi::c_void>(),
                        <Self as crate::DataType>::data_type(),
                    )
                }
            }
        }

        // SAFETY: We can transmute between our wrapper type and the contained inner type. This will
        // be ensured by using `#[repr(transparent)]` above.
        unsafe impl crate::DataType for $name {
            type Inner = open62541_sys::$inner;

            fn data_type() -> *const open62541_sys::UA_DataType {
                // SAFETY: We use this static variable only read-only.
                let types = unsafe { &open62541_sys::UA_TYPES };
                // PANIC: Value must fit into `usize` to allow indexing.
                let index = usize::try_from(open62541_sys::$index).unwrap();
                // PANIC: The given index must be valid within `UA_TYPES`.
                types.get(index).unwrap()
            }

            #[must_use]
            unsafe fn from_raw(src: Self::Inner) -> Self {
                $name(src)
            }

            #[must_use]
            fn into_raw(self) -> Self::Inner {
                // SAFETY: Move value out of `self` despite it not being `Copy`. We consume `self`
                // and forget it below, so that `Drop` is not called on the original value.
                let inner = unsafe { std::ptr::read(std::ptr::addr_of!(self.0)) };
                // Make sure that `drop()` is not called anymore.
                std::mem::forget(self);
                inner
            }
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                <Self as crate::DataType>::clone_raw(&self.0)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let output = crate::DataType::print(self);
                let string = output.as_ref().and_then(|output| output.as_str());
                write!(f, "{}({})", stringify!($name), string.unwrap_or("_"))
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let output = crate::DataType::print(self);
                let string = output.as_ref().and_then(|output| output.as_str());
                f.write_str(string.unwrap_or(stringify!($name)))
            }
        }
    };
}

pub(crate) use data_type;
