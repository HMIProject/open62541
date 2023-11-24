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
