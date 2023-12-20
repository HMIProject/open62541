use std::ffi::c_void;

use open62541_sys::{UA_print, UA_STATUSCODE_GOOD};

use crate::ua;

/// Transparent wrapper for OPC UA data type.
///
/// # Safety
///
/// We require that it must be possible to transmute between the type that implements `DataType` and
/// the wrapped type [`Self::Inner`]. Therefore, `#[repr(transparent)]` must be used when one wishes
/// to implement `DataType`.
pub unsafe trait DataType: Clone {
    /// Inner type.
    ///
    /// We require that it must be possible to transmute between the inner type and the wrapper type
    /// that implements `DataType`. This implies that `#[repr(transparent)]` must be set on any type
    /// that implements the `DataType` trait.
    type Inner;

    #[must_use]
    fn data_type() -> *const open62541_sys::UA_DataType;

    #[must_use]
    fn data_type_ref() -> &'static open62541_sys::UA_DataType {
        unsafe { Self::data_type().as_ref() }.unwrap()
    }

    /// Creates wrapper by cloning value from `src`.
    ///
    /// The original value must still be cleared with [`UA_clear()`] or deleted with [`UA_delete()`]
    /// if allocated on the heap, to avoid memory leaks. If `src` is borrowed from a second wrapper,
    /// that wrapper will make sure of this.
    ///
    /// [`UA_clear()`]: open62541_sys::UA_clear
    /// [`UA_delete()`]: open62541_sys::UA_delete
    #[must_use]
    fn from_ref(src: &Self::Inner) -> Self;

    #[must_use]
    fn as_ref(&self) -> &Self::Inner {
        // This transmutes the value into the inner type through `cast()`. Types that implement this
        // trait guarantee that we can transmute between them and their inner type, so this is okay.
        let ptr = (self as *const Self).cast::<Self::Inner>();
        // SAFETY: Dereferencing the pointer is allowed because of this transmutability.
        unsafe { ptr.as_ref().unwrap_unchecked() }
    }

    #[must_use]
    fn as_mut(&mut self) -> &mut Self::Inner {
        // This transmutes the value into the inner type through `cast()`. Types that implement this
        // trait guarantee that we can transmute between them and their inner type, so this is okay.
        let ptr = (self as *mut Self).cast::<Self::Inner>();
        // SAFETY: Dereferencing the pointer is allowed because of this transmutability.
        unsafe { ptr.as_mut().unwrap_unchecked() }
    }

    #[must_use]
    fn as_ptr(&self) -> *const Self::Inner {
        // This transmutes the value into the inner type through `cast()`. Types that implement this
        // trait guarantee that we can transmute between them and their inner type, so this is okay.
        (self as *const Self).cast::<Self::Inner>()
    }

    #[must_use]
    fn as_mut_ptr(&mut self) -> *mut Self::Inner {
        // This transmutes the value into the inner type through `cast()`. Types that implement this
        // trait guarantee that we can transmute between them and their inner type, so this is okay.
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

        if result != UA_STATUSCODE_GOOD {
            return None;
        }

        Some(output)
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
            /// Creates wrapper initialized with defaults.
            #[must_use]
            pub fn init() -> Self {
                let mut inner = <open62541_sys::$inner as std::default::Default>::default();
                // `UA_init()` depends on the specific data type, so we must call it, even though it
                // only zero-initializes the memory region (again) in most cases.
                unsafe {
                    open62541_sys::UA_init(
                        std::ptr::addr_of_mut!(inner).cast::<std::ffi::c_void>(),
                        <Self as crate::DataType>::data_type(),
                    )
                };
                Self(inner)
            }

            /// Creates wrapper by cloning value from `src`.
            ///
            /// The original value must still be cleared with [`UA_clear()`] or deleted with
            /// [`UA_delete()`] if allocated on the heap, to avoid memory leaks. If `src` is
            /// borrowed from a second wrapper, that wrapper will make sure of this.
            ///
            /// [`UA_clear()`]: open62541_sys::UA_clear
            /// [`UA_delete()`]: open62541_sys::UA_delete
            #[allow(dead_code)]
            #[must_use]
            pub(crate) fn from_ref(src: &open62541_sys::$inner) -> Self {
                // `UA_copy()` does not clean up the target before copying into it, so we may use an
                // uninitialized slice of memory here.
                let mut dst = <open62541_sys::$inner as std::default::Default>::default();

                let result = unsafe {
                    open62541_sys::UA_copy(
                        (src as *const open62541_sys::$inner).cast::<std::ffi::c_void>(),
                        std::ptr::addr_of_mut!(dst).cast::<std::ffi::c_void>(),
                        <Self as crate::DataType>::data_type(),
                    )
                };
                assert_eq!(result, open62541_sys::UA_STATUSCODE_GOOD);

                Self(dst)
            }

            /// Gives up ownership and returns inner value.
            ///
            /// The returned value must be cleared with [`UA_clear`](open62541_sys::UA_clear) to not
            /// leak any memory. Alternatively, it may be re-wrapped with [`new()`](Self::new) which
            /// then cleans up when dropped regularly.
            #[allow(dead_code)]
            #[must_use]
            pub(crate) fn into_inner(self) -> open62541_sys::$inner {
                // SAFETY: Move value out of `self` despite it not being `Copy`. This is okay: we do
                // consume `self` and forget it, so that `Drop` is not called on the original value,
                // avoiding duplicate memory de-allocation via `UA_clear()` in `drop()` below.
                let inner = unsafe { std::ptr::read(std::ptr::addr_of!(self.0)) };
                // Make sure that `drop()` is not called anymore.
                std::mem::forget(self);
                inner
            }

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
                // `UA_clear` resets the data structure, freeing any dynamically allocated memory in
                // it, no matter how deeply nested.
                unsafe {
                    open62541_sys::UA_clear(
                        std::ptr::addr_of_mut!(self.0).cast::<std::ffi::c_void>(),
                        <Self as crate::DataType>::data_type(),
                    )
                }
            }
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                Self::from_ref(&self.0)
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

        // SAFETY: We can transmute between our wrapper type and the contained inner type. This will
        // be ensured by using `#[repr(transparent)]` above.
        unsafe impl crate::DataType for $name {
            type Inner = open62541_sys::$inner;

            fn data_type() -> *const open62541_sys::UA_DataType {
                // SAFETY: We use this static variable only read-only.
                let types = unsafe { &open62541_sys::UA_TYPES };
                // PANIC: The given index must be valid within `UA_TYPES`.
                types.get(open62541_sys::$index as usize).unwrap()
            }

            fn from_ref(src: &Self::Inner) -> Self {
                $name::from_ref(src)
            }
        }
    };
}

pub(crate) use data_type;
