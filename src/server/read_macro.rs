/// Helper macro for `server_read` macro
#[allow(unused_macros)]
#[macro_export]
macro_rules! __read_operation {
    ($type:ty, $attr_id:expr, $server:expr, $node_id:expr) => {
        $server.read::<$type>($attr_id, $node_id)
    };
}

/// Reads an attribute from the specified node. Returns the value with the corresponding data type.
///
/// Make sure to also import the helper macro `__read_operation`
///
/// # Parameters
///
/// - `NODEID`: Reads a `NodeId` attribute.
/// - `NODECLASS`: Reads a `NodeClass` attribute.
/// - etc.
///
/// # Usage
///
/// `server_read!(ATTRIBUTEID, server: open62541::Server, node_id_value: &open62541::ua::NodeId);`
///
/// * `ATTRIBUTEID`: What attribute to read. Specify without `ua::AttributeId::` prefix.
/// * `server`: The server object on which `Server::read(...)` should be called.
/// * `node_id_value`: The specified attribute will be read from this node.
///
///
/// # Example
///
/// See `examples/server.rs:read_example()`
#[allow(unused_macro_rules)]
#[macro_export]
macro_rules! server_read {
    (NODEID, $server:expr, $node_id:expr) => {
        __read_operation!(ua::NodeId, ua::AttributeId::NODEID, $server, $node_id)
    };
    (NODECLASS, $server:expr, $node_id:expr) => {
        __read_operation!(ua::NodeClass, ua::AttributeId::NODECLASS, $server, $node_id)
    };
    (BROWSENAME, $server:expr, $node_id:expr) => {
        __read_operation!(
            ua::QualifiedName,
            ua::AttributeId::BROWSENAME,
            $server,
            $node_id
        )
    };
    (DISPLAYNAME, $server:expr, $node_id:expr) => {
        __read_operation!(
            ua::LocalizedText,
            ua::AttributeId::DISPLAYNAME,
            $server,
            $node_id
        )
    };
    (DESCRIPTION, $server:expr, $node_id:expr) => {
        __read_operation!(
            ua::LocalizedText,
            ua::AttributeId::DESCRIPTION,
            $server,
            $node_id
        )
    };
    (WRITEMASK, $server:expr, $node_id:expr) => {
        __read_operation!(ua::UInt32, ua::AttributeId::WRITEMASK, $server, $node_id)
    };
    (ISABSTRACT, $server:expr, $node_id:expr) => {
        __read_operation!(ua::Boolean, ua::AttributeId::ISABSTRACT, $server, $node_id)
    };
    (SYMMETRIC, $server:expr, $node_id:expr) => {
        __read_operation!(ua::Boolean, ua::AttributeId::SYMMETRIC, $server, $node_id)
    };
    (INVERSENAME, $server:expr, $node_id:expr) => {
        __read_operation!(
            ua::LocalizedText,
            ua::AttributeId::INVERSENAME,
            $server,
            $node_id
        )
    };
    (CONTAINSNOLOOPS, $server:expr, $node_id:expr) => {
        __read_operation!(
            ua::Boolean,
            ua::AttributeId::CONTAINSNOLOOPS,
            $server,
            $node_id
        )
    };
    (EVENTNOTIFIER, $server:expr, $node_id:expr) => {
        __read_operation!(ua::Byte, ua::AttributeId::EVENTNOTIFIER, $server, $node_id)
    };
    (VALUE, $server:expr, $node_id:expr) => {
        __read_operation!(ua::Variant, ua::AttributeId::VALUE, $server, $node_id)
    };
    (DATATYPE, $server:expr, $node_id:expr) => {
        __read_operation!(ua::NodeId, ua::AttributeId::DATATYPE, $server, $node_id)
    };
    (VALUERANK, $server:expr, $node_id:expr) => {
        __read_operation!(ua::Int32, ua::AttributeId::VALUERANK, $server, $node_id)
    };
    (ARRAYDIMENSIONS, $server:expr, $node_id:expr) => {
        __read_operation!(
            ua::Variant,
            ua::AttributeId::ARRAYDIMENSIONS,
            $server,
            $node_id
        )
    };
    (ACCESSLEVEL, $server:expr, $node_id:expr) => {
        __read_operation!(ua::Byte, ua::AttributeId::ACCESSLEVEL, $server, $node_id)
    };
    (MINIMUMSAMPLINGINTERVAL, $server:expr, $node_id:expr) => {
        __read_operation!(
            ua::Double,
            ua::AttributeId::MINIMUMSAMPLINGINTERVAL,
            $server,
            $node_id
        )
    };
    (HISTORIZING, $server:expr, $node_id:expr) => {
        __read_operation!(ua::Boolean, ua::AttributeId::HISTORIZING, $server, $node_id)
    };
    (EXECUTABLE, $server:expr, $node_id:expr) => {
        __read_operation!(ua::Boolean, ua::AttributeId::EXECUTABLE, $server, $node_id)
    };
    // Handle default case if necessary
    ($unknown:ident, $server:expr, $node_id:expr) => {{
        compile_error!(
            "No read available for ua::AttributeId: {:?}",
            stringify!($unknown)
        )
    }};
}
