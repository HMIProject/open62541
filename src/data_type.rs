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
pub(crate) unsafe trait DataType {
    /// Inner type.
    ///
    /// We require that it must be possible to transmute between the inner type and the wrapper type
    /// that implements `DataType`. This implies that `#[repr(transparent)]` must be set on any type
    /// that implements the `DataType` trait.
    type Inner;

    fn data_type() -> *const open62541_sys::UA_DataType;

    fn data_type_ref() -> &'static open62541_sys::UA_DataType {
        unsafe { Self::data_type().as_ref() }.unwrap()
    }

    #[must_use]
    fn as_ref(&self) -> &Self::Inner {
        // This transmutes the value into the inner type through `cast()`. Types that implement this
        // trait guarantee that we can transmute between them and their inner type, so this is okay.
        //
        // SAFETY: Dereferencing the pointer is allowed because of this transmutability.
        unsafe { &*(self as *const Self).cast::<Self::Inner>() }
    }

    #[must_use]
    fn as_mut(&mut self) -> &mut Self::Inner {
        // This transmutes the value into the inner type through `cast()`. Types that implement this
        // trait guarantee that we can transmute between them and their inner type, so this is okay.
        //
        // SAFETY: Dereferencing the pointer is allowed because of this transmutability.
        unsafe { &mut *(self as *mut Self).cast::<Self::Inner>() }
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
        let mut output = ua::String::default();

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
            /// Creates wrapper by taking ownership of `src`.
            #[allow(dead_code)]
            #[must_use]
            pub(crate) fn new(src: open62541_sys::$inner) -> Self {
                // This takes ownership of the wrapped value. We call `UA_clear()` when the value is
                // dropped eventually.
                Self(src)
            }

            /// Creates wrapper by cloning value from `src`.
            ///
            /// The original value must still be cleared with [`UA_clear`](open62541_sys::UA_clear),
            /// or deleted with [`UA_delete`](open62541_sys::UA_delete) if allocated on the heap, to
            /// avoid memory leaks. If the original value is only borrowed from another wrapper, the
            /// wrapper will make sure of this.
            #[allow(dead_code)]
            #[must_use]
            pub(crate) fn from_ref(src: &open62541_sys::$inner) -> Self {
                // `UA_copy()` does not clean up the target before copying into it, so we may use an
                // uninitialized slice of memory here.
                let mut dst = unsafe {
                    std::mem::MaybeUninit::<open62541_sys::$inner>::zeroed().assume_init()
                };

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

            #[allow(dead_code)]
            #[must_use]
            pub(crate) fn into_inner(self) -> open62541_sys::$inner {
                let inner = self.0;
                // Make sure that `drop()` is not called anymore.
                std::mem::forget(self);
                inner
            }
        }

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

        impl Default for $name {
            /// Creates wrapper initialized with defaults.
            fn default() -> Self {
                let mut inner = unsafe {
                    std::mem::MaybeUninit::<open62541_sys::$inner>::zeroed().assume_init()
                };
                unsafe {
                    open62541_sys::UA_init(
                        std::ptr::addr_of_mut!(inner).cast::<std::ffi::c_void>(),
                        <Self as crate::DataType>::data_type(),
                    )
                };
                Self(inner)
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
                f.write_fmt(format_args!(
                    "{}({})",
                    stringify!($name),
                    string.unwrap_or("_")
                ))
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
                // SAFETY: The given index must be valid within `UA_TYPES`.
                unsafe { open62541_sys::UA_TYPES.get(open62541_sys::$index as usize) }.unwrap()
            }
        }
    };
}

pub(crate) use data_type;
