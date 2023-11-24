mod array;
mod client;
mod data_value;
mod node_id;
mod read_request;
mod read_response;
mod read_value_id;
mod string;
mod variant;

use std::ptr::NonNull;

use open62541_sys::{UA_DataType, UA_TYPES};

pub use self::{
    array::Array, client::Client, data_value::DataValue, node_id::NodeId,
    read_request::ReadRequest, read_response::ReadResponse, read_value_id::ReadValueId,
    string::String, variant::Variant,
};

pub trait DataType {
    type Inner;

    /// Index into `UA_TYPES`.
    const INNER: u32;

    #[must_use]
    fn inner() -> NonNull<UA_DataType> {
        NonNull::from(unsafe { &UA_TYPES[Self::INNER as usize] })
    }

    fn as_ptr(&self) -> *const Self::Inner;
}

macro_rules! data_type {
    ($name:ident, $inner:ident, $index:ident) => {
        pub struct $name(open62541_sys::$inner);

        impl $name {
            fn data_type() -> *const open62541_sys::UA_DataType {
                unsafe { open62541_sys::UA_TYPES.get(open62541_sys::$index as usize) }.unwrap()
            }

            #[must_use]
            pub fn new() -> Self {
                let mut inner = unsafe {
                    std::mem::MaybeUninit::<open62541_sys::$inner>::zeroed().assume_init()
                };
                unsafe {
                    open62541_sys::UA_init(std::ptr::addr_of_mut!(inner).cast(), Self::data_type())
                };
                Self(inner)
            }

            /// Copies value from `src`.
            #[allow(dead_code)]
            #[must_use]
            pub(crate) fn from(src: &open62541_sys::$inner) -> Self {
                let mut dst = Self::new();

                let result = unsafe {
                    open62541_sys::UA_copy(
                        std::ptr::addr_of!(*src).cast(),
                        std::ptr::addr_of_mut!(dst).cast(),
                        Self::data_type(),
                    )
                };
                assert_eq!(result, open62541_sys::UA_STATUSCODE_GOOD);

                dst
            }

            /// Takes ownership of `src`.
            #[allow(dead_code)]
            #[must_use]
            pub(crate) fn from_inner(src: open62541_sys::$inner) -> Self {
                Self(src)
            }

            #[allow(dead_code)]
            #[must_use]
            pub(crate) const fn as_ref(&self) -> &open62541_sys::$inner {
                &self.0
            }

            #[allow(dead_code)]
            #[must_use]
            pub(crate) fn as_mut(&mut self) -> &mut open62541_sys::$inner {
                &mut self.0
            }

            #[allow(dead_code)]
            #[must_use]
            pub(crate) const fn as_ptr(&self) -> *const open62541_sys::$inner {
                std::ptr::addr_of!(self.0)
            }

            #[allow(dead_code)]
            #[must_use]
            pub(crate) fn as_mut_ptr(&mut self) -> *mut open62541_sys::$inner {
                std::ptr::addr_of_mut!(self.0)
            }

            #[allow(dead_code)]
            #[must_use]
            pub(crate) fn into_inner(self) -> open62541_sys::$inner {
                let inner = self.0;
                std::mem::forget(self);
                inner
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                unsafe {
                    open62541_sys::UA_clear(std::ptr::addr_of_mut!(*self).cast(), Self::data_type())
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                Self::from(&self.0)
            }
        }
    };
}

pub(crate) use data_type;
