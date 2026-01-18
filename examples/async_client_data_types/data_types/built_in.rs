//! - [Part 3: 3.1.3 Built-in DataType](https://reference.opcfoundation.org/Core/Part3/v105/docs/3.1.3)
//! - [Part 6: 5.1.2 Built-in Types](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.2)

mod boolean;
mod byte_string;
mod data_value;
mod date_time;
mod diagnostic_info;
mod expanded_node_id;
mod extension_object;
mod guid;
mod localized_text;
mod node_id;
mod number;
mod qualified_name;
mod status_code;
mod string;
mod variant;
mod xml_element;

pub use self::{
    boolean::Boolean,
    byte_string::ByteString,
    data_value::DataValue,
    date_time::DateTime,
    diagnostic_info::DiagnosticInfo,
    expanded_node_id::ExpandedNodeId,
    extension_object::ExtensionObject,
    guid::Guid,
    localized_text::LocalizedText,
    node_id::NodeId,
    number::{
        Byte, Decimal, Double, Float, Index, Int16, Int32, Int64, Integer, Number, SByte, UInt16,
        UInt32, UInt64, UInteger,
    },
    qualified_name::QualifiedName,
    status_code::StatusCode,
    string::{LocaleId, String},
    variant::{Variant, VariantArray, VariantScalar},
    xml_element::XmlElement,
};

// [Part 3: 8.7 BaseDataType](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.7)
pub trait BaseDataType {}
