mod client;
mod error;
pub mod ua;

pub use self::{
    client::{Client, ClientBuilder},
    error::Error,
};

pub(crate) trait DataType {
    type Inner;

    fn as_ptr(&self) -> *const Self::Inner;

    fn data_type() -> *const open62541_sys::UA_DataType;

    fn data_type_ref() -> &'static open62541_sys::UA_DataType {
        unsafe { Self::data_type().as_ref() }.unwrap()
    }
}
