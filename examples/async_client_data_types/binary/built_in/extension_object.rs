use bytes::Bytes;

use crate::{
    binary::StatelessBinaryReader,
    data_types::{
        Byte, ByteString, ExtensionObject, NodeId, StructureDefinition, StructureType, XmlElement,
    },
};

impl ExtensionObject {
    pub fn decode_structure(definition: &StructureDefinition) {
        match definition.structure_type {
            StructureType::Structure => todo!(),
            StructureType::StructureWithOptionalFields => todo!(),
            StructureType::StructureWithSubtypedValues => todo!(),
            StructureType::Union => todo!(),
            StructureType::UnionWithSubtypedValues => todo!(),
        }

        for field in definition.fields {}
    }
}

// [Part 6: 5.2.2.15 ExtensionObject](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.15)
impl StatelessBinaryReader for ExtensionObject {
    fn read(data: &mut Bytes) -> Self {
        let type_id = NodeId::read(data);
        let encoding = Byte::read(data);

        match encoding.0 {
            0x00 => Self::Null(type_id),
            0x01 => {
                let body = ByteString::read(data);
                Self::ByteString(type_id, body)
            }
            0x02 => {
                let body = XmlElement::read(data);
                Self::XmlElement(type_id, body)
            }
            _ => panic!(),
        }
    }
}
