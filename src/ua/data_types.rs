//! Thin wrappers for OPC UA data types from [`open62541_sys`].

mod data_value;
mod node_id;
mod read_request;
mod read_response;
mod read_value_id;
mod string;
mod variant;

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
            #[allow(dead_code)]
            #[must_use]
            fn data_type() -> *const open62541_sys::UA_DataType {
                unsafe { open62541_sys::UA_TYPES.get(open62541_sys::$index as usize) }.unwrap()
            }

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
            pub(crate) fn new_from(src: &open62541_sys::$inner) -> Self {
                // `UA_copy()` does not clean up the target before copying into it, so we may use an
                // uninitialized slice of memory here.
                let mut dst = unsafe {
                    std::mem::MaybeUninit::<open62541_sys::$inner>::zeroed().assume_init()
                };

                let result = unsafe {
                    open62541_sys::UA_copy(
                        (src as *const open62541_sys::$inner).cast::<std::ffi::c_void>(),
                        std::ptr::addr_of_mut!(dst).cast::<std::ffi::c_void>(),
                        Self::data_type(),
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
                        Self::data_type(),
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
                        Self::data_type(),
                    )
                };
                Self(inner)
            }
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                Self::new_from(&self.0)
            }
        }

        unsafe impl crate::DataType for $name {
            type Inner = open62541_sys::$inner;

            fn data_type() -> *const open62541_sys::UA_DataType {
                $name::data_type()
            }
        }
    };
}

pub(crate) use data_type;

pub use self::{
    data_value::DataValue, node_id::NodeId, read_request::ReadRequest, read_response::ReadResponse,
    read_value_id::ReadValueId, string::String, variant::Variant,
};
