//! Thin wrappers for [`open62541_sys`] types.

mod array;
mod client;
mod data_types;
mod status_code;

use std::ptr::NonNull;

use open62541_sys::{UA_DataType, UA_TYPES};

pub use self::{array::Array, client::Client, data_types::*, status_code::StatusCode};

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
