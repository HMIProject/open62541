mod client;
mod error;
pub mod ua;

pub use self::{
    client::{Client, ClientBuilder},
    error::Error,
};

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
}
