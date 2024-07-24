use crate::data_type::DataType;
use crate::traits::Readable;
use crate::ua;
use crate::Error;
use crate::Result;
use open62541_sys::{UA_Server, __UA_Server_read};

// Macro to implement the Readable trait for each type
macro_rules! impl_readable {
    // This handles both single and multiple attribute IDs
    ($($type:ty => [$($variant:ident),+]),+ $(,)?) => {
        $(
            impl Readable for $type {
                fn read(server: *const UA_Server, attribute_id: ua::AttributeId, node_id: &ua::NodeId) -> Result<Self> {
                    if !($(attribute_id == ua::AttributeId::$variant)&&+) {
                        return Err(Error::Internal("Invalid attribute id for this type!"));
                    }
                    let mut value: Self = unsafe { std::mem::zeroed() };
                    #[allow(trivial_casts)]
                    let status_code = ua::StatusCode::new(unsafe {
                        __UA_Server_read(server.cast_mut(), node_id.as_ptr(), attribute_id.into_raw(), value.as_mut_ptr().cast())});

                    Error::verify_good(&status_code)?;
                    Ok(value)
                }
            }
        )+
    };
}

// This macro generates implementations of trait `Readable`
// for all these types and uses the specified `AttributeId`s
// to check if the user-specified datatype matches the one
// that will be returned by the call to `__UA_Server_read()`
// This check is required when the crate user directly calls
// `Server::read()` without using the read! macro.
impl_readable!(
    ua::NodeId => [NODEID, DATATYPE],
    ua::NodeClass => [NODECLASS],
    ua::QualifiedName => [BROWSENAME],
    ua::UInt32 => [WRITEMASK],
    ua::LocalizedText => [DISPLAYNAME, DESCRIPTION, INVERSENAME],
    ua::Byte => [EVENTNOTIFIER, DESCRIPTION, ACCESSLEVEL],
    ua::Variant => [VALUE, ARRAYDIMENSIONS],
    ua::Boolean => [ISABSTRACT, SYMMETRIC, CONTAINSNOLOOPS, HISTORIZING, EXECUTABLE],
    ua::Int32 => [VALUERANK],
    ua::Double => [MINIMUMSAMPLINGINTERVAL]
);
