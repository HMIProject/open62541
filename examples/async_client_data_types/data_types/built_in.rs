//! - [Part 3: 3.1.3 Built-in DataType](https://reference.opcfoundation.org/Core/Part3/v105/docs/3.1.3)
//! - [Part 6: 5.1.2 Built-in Types](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.2)

mod boolean;
mod byte;
mod byte_string;
mod data_value;
mod date_time;
mod diagnostic_info;
mod double;
mod expanded_node_id;
mod extension_object;
mod float;
mod guid;
mod int16;
mod int32;
mod int64;
mod localized_text;
mod node_id;
mod qualified_name;
mod sbyte;
mod status_code;
mod string;
mod uint16;
mod uint32;
mod uint64;
mod variant;
mod xml_element;

pub use self::{
    boolean::Boolean,
    byte::Byte,
    byte_string::ByteString,
    data_value::DataValue,
    date_time::DateTime,
    diagnostic_info::DiagnosticInfo,
    double::Double,
    expanded_node_id::ExpandedNodeId,
    extension_object::ExtensionObject,
    float::Float,
    guid::Guid,
    int16::Int16,
    int32::Int32,
    int64::Int64,
    localized_text::LocalizedText,
    node_id::NodeId,
    qualified_name::QualifiedName,
    sbyte::SByte,
    status_code::StatusCode,
    string::String,
    uint16::UInt16,
    uint32::UInt32,
    uint64::UInt64,
    variant::{Variant, VariantArray, VariantScalar},
    xml_element::XmlElement,
};
