use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr as _,
};

use anyhow::Context as _;
use itertools::Itertools;
use open62541::{
    AsyncClient, ClientBuilder, DataType,
    ua::{self, DataTypeArray},
};
use open62541_sys::{
    UA_NS0ID_BASEDATATYPE, UA_NS0ID_HASSUBTYPE, UA_NS0ID_PUBLISHSUBSCRIBETYPE_ADDCONNECTION,
    UA_NS0ID_REFERENCELISTENTRYDATATYPE, UA_NS0ID_VARIABLETYPENODE_ENCODING_DEFAULTJSON,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    for fetch_upfront in [true, false] {
        read_value(fetch_upfront).await?;
    }

    println!("Exiting");

    Ok(())
}

async fn read_value(fetch_upfront: bool) -> anyhow::Result<()> {
    let mut client = ClientBuilder::default()
        .connect("opc.tcp://10.92.40.124:4841")
        .context("connect")?;
    if fetch_upfront {
        client = client
            .with_remote_data_types()
            .context("get remote data types")?
    }
    let mut client = client.into_async();

    println!("Connected successfully (fetch_upfront: {fetch_upfront})");

    // let node_id = ua::NodeId::from_str("ns=3;s=\"6601_HMI\".\"Freshwater\"").unwrap();
    // let node_id = ua::NodeId::from_str("ns=3;s=\"6601_HMI\".\"Filling\"").unwrap();
    let node_id = ua::NodeId::from_str("ns=3;s=\"6601_HMI\".\"M901\"").unwrap();

    let value = client
        .read_value(&node_id)
        .await
        .context("read value")?
        .into_value()
        .context("turn into value")?;
    println!("Raw value: {value:?}");

    let type_id = value.type_id().context("get type ID")?;
    println!("Type ID: {type_id:?}");

    if let Some(mut extension_object_value) = value.to_scalar::<ua::ExtensionObject>() {
        let data_type_id = client
            .read_attribute(&node_id, ua::AttributeId::DATATYPE_T)
            .await
            .context("read data type")?
            .into_scalar_value()
            .context("turn into scalar value")?;

        let mut data_type_descriptions =
            read_nested_data_type_descriptions(&client, &[data_type_id], &[]).await?;
        println!("Data type descriptions: {data_type_descriptions:?}");

        let data_type_ids = data_type_descriptions
            .iter()
            .map(|data_type_description| data_type_description.data_type_id().to_owned())
            .collect::<BTreeSet<_>>();

        let mut missing_types = BTreeSet::new();

        for data_type_description in &mut data_type_descriptions {
            let ua::DataTypeDescription::Structure(structure_description) = data_type_description
            else {
                unimplemented!();
            };

            for field in structure_description
                .structure_definition()
                .fields()
                .unwrap()
                .iter_mut()
            {
                let data_type_id = field.data_type();
                if !data_type_id.is_ns0() && !data_type_ids.contains(data_type_id) {
                    println!("MISSING: {data_type_id:?}");
                    missing_types.insert(data_type_id.to_owned());
                }
            }
        }

        if !missing_types.is_empty() {
            let supertypes = read_supertypes(&client, &missing_types).await?;
            println!("Supertypes: {supertypes:?}");
            for (subtype, supertype) in missing_types.into_iter().zip(supertypes) {
                for data_type_description in data_type_descriptions.iter_mut().skip(1) {
                    data_type_description.replace_data_type(&subtype, &supertype);
                    // data_type_description
                    //     .replace_data_type(&subtype, &ua::NodeId::ns0(UA_NS0ID_BASEDATATYPE));
                }
            }
            for data_type_description in &mut data_type_descriptions {
                data_type_description.drop_arrays();
            }
            println!("Adjusted data type descriptions: {data_type_descriptions:?}");
        }

        let number_of_new_data_types = client.add_data_types(&data_type_descriptions)?;
        println!("Added {number_of_new_data_types} new data types");

        println!("Encoded value: {extension_object_value:?}");
        client.decode_extension_object(&mut extension_object_value)?;
        println!("Decoded value: {extension_object_value:?}");
        // let member =
        //     data_type.get_struct_member(&mut extension_object_value, "active_flushing2")?;
        // println!("Member value: {member:?}");
        // println!("Decoded value: {extension_object_value:?}");
    }

    let value = client
        .read_value(&node_id)
        .await
        .context("read value")?
        .into_value()
        .context("turn into value")?;
    println!("Raw value: {value:?}");

    let type_id = value.type_id().context("get type ID")?;
    println!("Type ID: {type_id:?}");

    println!("Disconnecting client");

    client.disconnect().await;

    Ok(())
}

async fn read_supertypes(
    client: &AsyncClient,
    data_type_ids: impl IntoIterator<Item = &ua::NodeId>,
) -> anyhow::Result<Vec<ua::NodeId>> {
    let browse_descriptions = data_type_ids
        .into_iter()
        .map(|data_type_id| {
            ua::BrowseDescription::init()
                .with_node_id(data_type_id)
                .with_browse_direction(&ua::BrowseDirection::INVERSE)
                .with_reference_type_id(&ua::NodeId::ns0(UA_NS0ID_HASSUBTYPE))
                .with_include_subtypes(true)
                .with_node_class_mask(&ua::NodeClassMask::DATATYPE)
                .with_result_mask(&ua::BrowseResultMask::NONE)
        })
        .collect::<Vec<_>>();

    let results = client.browse_many(&browse_descriptions).await?;

    let mut supertypes = Vec::with_capacity(results.len());

    for result in results {
        let result = result?;

        if !result.1.is_none() {
            unimplemented!();
        };

        let nodes = result.0.to_vec();
        assert_eq!(nodes.len(), 1);

        supertypes.push(nodes.first().unwrap().node_id().node_id().to_owned());
    }

    Ok(supertypes)
}

async fn read_nested_data_type_descriptions(
    client: &AsyncClient,
    data_type_ids: &[ua::NodeId],
    known_data_type_ids: &[ua::NodeId],
) -> anyhow::Result<Vec<ua::DataTypeDescription>> {
    enum KnownDataType {
        PriorKnowledge,
        FoundNow(ua::DataTypeDescription),
    }

    const MAX_READ_DATA_TYPE_DESCRIPTION_LEN: usize = 100;

    let mut known_data_types =
        BTreeMap::from_iter(known_data_type_ids.iter().map(|known_data_type_id| {
            (known_data_type_id.to_owned(), KnownDataType::PriorKnowledge)
        }));
    let mut pending_data_type_ids = BTreeSet::from_iter(
        data_type_ids
            .iter()
            .filter(|data_type_id| !is_well_known_data_type(data_type_id))
            .cloned(),
    );

    while !pending_data_type_ids.is_empty() {
        let data_type_ids = pending_data_type_ids
            .iter()
            .take(MAX_READ_DATA_TYPE_DESCRIPTION_LEN)
            .collect::<Vec<_>>();

        let data_type_descriptions = read_data_type_descriptions(client, &data_type_ids).await?;

        for data_type_id in find_nested_data_type_ids(&data_type_descriptions)? {
            if !known_data_type_ids.contains(&data_type_id)
                && !is_well_known_data_type(&data_type_id)
            {
                pending_data_type_ids.insert(data_type_id);
            }
        }

        for data_type_description in data_type_descriptions {
            let data_type_id = data_type_description.data_type_id();

            pending_data_type_ids.remove(data_type_id);
            known_data_types.insert(
                data_type_id.to_owned(),
                KnownDataType::FoundNow(data_type_description),
            );
        }
    }

    Ok(known_data_types
        .into_values()
        .filter_map(|known_data_type| match known_data_type {
            KnownDataType::PriorKnowledge => None,
            KnownDataType::FoundNow(data_type_description) => Some(data_type_description),
        })
        .collect())
}

async fn read_data_type_descriptions(
    client: &AsyncClient,
    data_type_ids: &[&ua::NodeId],
) -> anyhow::Result<Vec<ua::DataTypeDescription>> {
    let node_attributes = data_type_ids
        .iter()
        .flat_map(|&data_type_id| {
            [
                // Match attributes with processing below.
                (data_type_id.to_owned(), ua::AttributeId::BROWSENAME),
                (data_type_id.to_owned(), ua::AttributeId::DATATYPEDEFINITION),
            ]
        })
        .collect::<Vec<_>>();

    println!(
        "Reading {} data type definitions: {:?}",
        data_type_ids.len(),
        data_type_ids
    );
    let attribute_values = client.read_many_attributes(&node_attributes).await?;

    let mut data_type_descriptions = Vec::with_capacity(data_type_ids.len());

    // Match attributes with read request above.
    for (&data_type_id, (browse_name, data_type_definition)) in data_type_ids
        .iter()
        .zip(attribute_values.into_iter().tuples())
    {
        let browse_name = browse_name
            .into_scalar_value()
            .context("require browse name value")?
            .into_scalar::<ua::QualifiedName>()
            .context("unexpected browse name type")?;

        let data_type_definition = ua::DataTypeDefinition::from_abstract(
            data_type_definition
                .into_scalar_value()
                .context("require data type definition value")?
                .into_scalar::<ua::Variant>()
                .context("unexpected data type definition type")?,
        )?;

        let structure_definition = match data_type_definition {
            ua::DataTypeDefinition::Structure(definition) => definition,
            ua::DataTypeDefinition::Enum(_) => {
                anyhow::bail!("unsupported enum definition")
            }
            _ => anyhow::bail!("unsupported data type definition"),
        };

        let structure_description =
            structure_definition.into_description(data_type_id.to_owned(), browse_name);

        data_type_descriptions.push(ua::DataTypeDescription::Structure(structure_description));
    }

    Ok(data_type_descriptions)
}

fn find_nested_data_type_ids(
    data_type_descriptions: &[ua::DataTypeDescription],
) -> anyhow::Result<BTreeSet<ua::NodeId>> {
    let mut nested_data_type_ids = BTreeSet::new();

    for data_type_description in data_type_descriptions {
        match data_type_description {
            ua::DataTypeDescription::Structure(description) => {
                let definition = description.structure_definition();
                let fields = definition.fields().context("missing struct fields")?;
                for field in fields.iter() {
                    nested_data_type_ids.insert(field.data_type().to_owned());
                }
            }
            ua::DataTypeDescription::Enum(_) => {
                anyhow::bail!("unsupported enum description")
            }
            _ => anyhow::bail!("unsupported data type description"),
        }
    }

    Ok(nested_data_type_ids)
}

fn is_well_known_data_type(data_type_id: &ua::NodeId) -> bool {
    // TODO: Add proper support for Simatic data types.
    data_type_id.is_ns0() || (data_type_id.namespace_index() == 3 && data_type_id.is_numeric())
}

// [5.2 OPC UA Binary](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2)
mod foo {
    use std::{num::NonZeroU32, string::String as StdString};

    use bytes::{Buf as _, Bytes};

    pub trait Binary {
        fn read(data: &mut Bytes) -> Self;
    }

    // [5.1.2 Built-in Types](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.2)
    // [5.2.2.1 Boolean](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.1)
    pub struct Boolean(bool);
    impl Binary for Boolean {
        fn read(data: &mut Bytes) -> Self {
            Self(data.try_get_u8().unwrap() != 0)
        }
    }

    // [5.1.2 Built-in Types](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.2)
    // [5.2.2.2 Integer](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.2)
    pub struct SByte(i8);
    impl Binary for SByte {
        fn read(data: &mut Bytes) -> Self {
            Self(data.try_get_i8().unwrap())
        }
    }
    pub struct Byte(u8);
    impl Binary for Byte {
        fn read(data: &mut Bytes) -> Self {
            Self(data.try_get_u8().unwrap())
        }
    }
    pub struct Int16(i16);
    impl Binary for Int16 {
        fn read(data: &mut Bytes) -> Self {
            Self(data.try_get_i16_le().unwrap())
        }
    }
    pub struct UInt16(u16);
    impl Binary for UInt16 {
        fn read(data: &mut Bytes) -> Self {
            Self(data.try_get_u16_le().unwrap())
        }
    }
    pub struct Int32(i32);
    impl Binary for Int32 {
        fn read(data: &mut Bytes) -> Self {
            Self(data.try_get_i32_le().unwrap())
        }
    }
    pub struct UInt32(u32);
    impl Binary for UInt32 {
        fn read(data: &mut Bytes) -> Self {
            Self(data.try_get_u32_le().unwrap())
        }
    }
    pub struct Int64(i64);
    impl Binary for Int64 {
        fn read(data: &mut Bytes) -> Self {
            Self(data.try_get_i64_le().unwrap())
        }
    }
    pub struct UInt64(u64);
    impl Binary for UInt64 {
        fn read(data: &mut Bytes) -> Self {
            Self(data.try_get_u64_le().unwrap())
        }
    }

    // [5.1.2 Built-in Types](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.2)
    // [5.2.2.3 Floating Point](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.3)
    pub struct Float(f32);
    impl Binary for Float {
        fn read(data: &mut Bytes) -> Self {
            Self(data.try_get_f32_le().unwrap())
        }
    }
    pub struct Double(f64);
    impl Binary for Double {
        fn read(data: &mut Bytes) -> Self {
            Self(data.try_get_f64_le().unwrap())
        }
    }

    // [5.1.2 Built-in Types](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.2)
    // [5.2.2.4 String](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.4)
    // [5.1.11 Null, Empty and Zero-Length Arrays](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.11)
    pub struct String(Option<StdString>);
    impl Binary for String {
        fn read(data: &mut Bytes) -> Self {
            let length = Int32::read(data);
            if length.0 == -1 {
                return Self(None);
            }
            let length = usize::try_from(length.0).unwrap();
            let mut bytes = vec![0; length];
            data.try_copy_to_slice(&mut bytes).unwrap();
            Self(Some(StdString::from_utf8(bytes).unwrap()))
        }
    }

    // [5.1.2 Built-in Types](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.2)
    // [5.1.3 Guid](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.3)
    // [5.2.2.6 Guid](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.6)
    pub struct Guid(u32, u16, u16, [u8; 8]);
    impl Binary for Guid {
        fn read(data: &mut Bytes) -> Self {
            let a = UInt32::read(data);
            let b = UInt16::read(data);
            let c = UInt16::read(data);
            let d = [
                Byte::read(data),
                Byte::read(data),
                Byte::read(data),
                Byte::read(data),
                Byte::read(data),
                Byte::read(data),
                Byte::read(data),
                Byte::read(data),
            ];
            Self(a.0, b.0, c.0, d.map(|byte| byte.0))
        }
    }

    // [5.1.2 Built-in Types](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.2)
    // [5.1.5 ByteString](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.5)
    // [5.2.2.7 ByteString](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.7)
    // [5.1.11 Null, Empty and Zero-Length Arrays](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.11)
    pub struct ByteString(Option<Vec<u8>>);
    impl Binary for ByteString {
        fn read(data: &mut Bytes) -> Self {
            let length = Int32::read(data);
            if length.0 == -1 {
                return Self(None);
            }
            let length = usize::try_from(length.0).unwrap();
            let mut bytes = vec![0; length];
            data.try_copy_to_slice(&mut bytes).unwrap();
            Self(Some(bytes))
        }
    }

    // [5.2.2.8 XmlElement (Deprecated)](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.8)
    pub struct XmlElement(Option<StdString>);
    impl Binary for XmlElement {
        fn read(data: &mut Bytes) -> Self {
            let string = ByteString::read(data);
            Self(string.0.map(|string| StdString::from_utf8(string).unwrap()))
        }
    }

    // [5.1.2 Built-in Types](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.2)
    // [5.1.12 QualifiedName, NodeId and ExpandedNodeId String Encoding](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.12)
    // [5.2.2.9 NodeId](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.9)
    pub enum NodeId {
        Numeric(u16, UInteger),
        String(u16, String),
        Guid(u16, Guid),
        Opaque(u16, ByteString),
    }
    impl Binary for NodeId {
        fn read(data: &mut Bytes) -> Self {
            let data_encoding = Byte::read(data);
            assert!(data_encoding.0 & 0x80 == 0x00);
            assert!(data_encoding.0 & 0x40 == 0x00);
            match data_encoding.0 {
                0x00 => {
                    let identifier = Byte::read(data);
                    Self::Numeric(0, UInteger::Byte(identifier))
                }
                0x01 => {
                    let namespace = Byte::read(data);
                    let identifier = UInt16::read(data);
                    Self::Numeric(u16::from(namespace.0), UInteger::UInt16(identifier))
                }
                0x02 => {
                    let namespace = UInt16::read(data);
                    let identifier = UInt32::read(data);
                    Self::Numeric(namespace.0, UInteger::UInt32(identifier))
                }
                0x03 => {
                    let namespace = UInt16::read(data);
                    let identifier = String::read(data);
                    Self::String(namespace.0, identifier)
                }
                0x04 => {
                    let namespace = UInt16::read(data);
                    let identifier = Guid::read(data);
                    Self::Guid(namespace.0, identifier)
                }
                0x05 => {
                    let namespace = UInt16::read(data);
                    let identifier = ByteString::read(data);
                    Self::Opaque(namespace.0, identifier)
                }
                _ => {
                    panic!();
                }
            }
        }
    }

    // [5.1.2 Built-in Types](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.2)
    // [5.1.12 QualifiedName, NodeId and ExpandedNodeId String Encoding](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.12)
    // [5.2.2.13 QualifiedName](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.13)
    pub struct QualifiedName(u16, String);
    impl Binary for QualifiedName {
        fn read(data: &mut Bytes) -> Self {
            let namespace_index = UInt16::read(data);
            let name = String::read(data);
            Self(namespace_index.0, name)
        }
    }

    // [5.1.2 Built-in Types](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.2)
    // [5.2.2.14 LocalizedText](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.14)
    pub struct LocalizedText {
        locale: Option<String>,
        text: Option<String>,
    }
    impl Binary for LocalizedText {
        fn read(data: &mut Bytes) -> Self {
            let encoding_mask = Byte::read(data);
            let locale = (encoding_mask.0 & 0x01 != 0x00).then(|| String::read(data));
            let text = (encoding_mask.0 & 0x02 != 0x00).then(|| String::read(data));
            Self { locale, text }
        }
    }

    // [5.1.2 Built-in Types](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.2)
    // [5.2.2.15 ExtensionObject](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.15)
    pub enum ExtensionObject {
        Null { type_id: NodeId },
        ByteString { type_id: NodeId, body: ByteString },
        XmlElement { type_id: NodeId, body: XmlElement },
    }
    impl Binary for ExtensionObject {
        fn read(data: &mut Bytes) -> Self {
            let type_id = NodeId::read(data);
            let encoding = Byte::read(data);
            match encoding.0 {
                0x00 => Self::Null { type_id },
                0x01 => {
                    let body = ByteString::read(data);
                    Self::ByteString { type_id, body }
                }
                0x02 => {
                    let body = XmlElement::read(data);
                    Self::XmlElement { type_id, body }
                }
                _ => panic!(),
            }
        }
    }

    // [5.1.9 Variant](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.9)
    // [5.2.2.16 Variant](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.16)
    // [5.1.11 Null, Empty and Zero-Length Arrays](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.11)
    pub enum Variant {
        Null,
        Scalar(VariantScalar),
        Array(Option<VariantArray>, Option<Vec<NonZeroU32>>),
    }
    impl Binary for Variant {
        fn read(data: &mut Bytes) -> Self {
            let encoding_mask = Byte::read(data);
            let type_id = encoding_mask.0 & 0b0011_1111;
            let array_dimensions = encoding_mask.0 & 0b0100_0000 != 0x00;
            let array_of_values = encoding_mask.0 & 0b1000_0000 != 0x00;
            if type_id == 0 {
                return Self::Null;
            }
            assert!(1 <= type_id && type_id <= 31);
            let type_id = BuiltInTypeId::new(type_id).unwrap_or(BuiltInTypeId::ByteString);
            if array_of_values {
                let array_length = Int32::read(data);
                let values = if array_length.0 == -1 {
                    None
                } else {
                    let array_length = usize::try_from(array_length.0).unwrap();
                    Some(VariantArray::read(type_id, array_length, data))
                };
                let array_dimensions = array_dimensions.then(|| {
                    let Array(Some(array_dimensions)) = Array::<Int32>::read(data) else {
                        panic!();
                    };
                    assert!(array_dimensions.len() >= 2);
                    let array_dimensions = array_dimensions
                        .into_iter()
                        .map(|array_dimension| {
                            let array_dimension = u32::try_from(array_dimension.0).unwrap();
                            NonZeroU32::new(array_dimension).unwrap()
                        })
                        .collect::<Vec<_>>();
                    assert!(
                        array_dimensions
                            .iter()
                            .map(|array_dimension| usize::try_from(array_dimension.get()).unwrap())
                            .product::<usize>()
                            == values.as_ref().map_or(0, VariantArray::len)
                    );
                    array_dimensions
                });
                Self::Array(values, array_dimensions)
            } else {
                let value = VariantScalar::read(type_id, data);
                assert!(!array_dimensions);
                Self::Scalar(value)
            }
        }
    }

    // [5.1.2 Built-in Types](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.2)
    #[derive(Clone, Copy)]
    enum BuiltInTypeId {
        Boolean,
        SByte,
        Byte,
        Int16,
        UInt16,
        Int32,
        UInt32,
        Int64,
        UInt64,
        Float,
        Double,
        String,
        DateTime,
        Guid,
        ByteString,
        XmlElement,
        NodeId,
        ExpandedNodeId,
        StatusCode,
        QualifiedName,
        LocalizedText,
        ExtensionObject,
        DataValue,
        Variant,
        DiagnosticInfo,
    }
    impl BuiltInTypeId {
        fn new(type_id: u8) -> Option<Self> {
            Some(match type_id {
                1 => Self::Boolean,
                2 => Self::SByte,
                3 => Self::Byte,
                4 => Self::Int16,
                5 => Self::UInt16,
                6 => Self::Int32,
                7 => Self::UInt32,
                8 => Self::Int64,
                9 => Self::UInt64,
                10 => Self::Float,
                11 => Self::Double,
                12 => Self::String,
                13 => Self::DateTime,
                14 => Self::Guid,
                15 => Self::ByteString,
                16 => Self::XmlElement,
                17 => Self::NodeId,
                18 => Self::ExpandedNodeId,
                19 => Self::StatusCode,
                20 => Self::QualifiedName,
                21 => Self::LocalizedText,
                22 => Self::ExtensionObject,
                23 => Self::DataValue,
                24 => Self::Variant,
                25 => Self::DiagnosticInfo,
                _ => return None,
            })
        }
    }

    // [5.1.2 Built-in Types](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.2)
    enum VariantScalar {
        Boolean(Boolean),
        SByte(SByte),
        Byte(Byte),
        Int16(Int16),
        UInt16(UInt16),
        Int32(Int32),
        UInt32(UInt32),
        Int64(Int64),
        UInt64(UInt64),
        Float(Float),
        Double(Double),
        String(String),
        // DateTime(DateTime),
        Guid(Guid),
        ByteString(ByteString),
        XmlElement(XmlElement),
        NodeId(NodeId),
        // ExpandedNodeId(ExpandedNodeId),
        // StatusCode(StatusCode),
        QualifiedName(QualifiedName),
        LocalizedText(LocalizedText),
        ExtensionObject(ExtensionObject),
        // DataValue(DataValue),
        // // Variant(Variant),
        // // DiagnosticInfo(DiagnosticInfo),
    }
    impl VariantScalar {
        fn read(type_id: BuiltInTypeId, data: &mut Bytes) -> Self {
            macro_rules! read {
                ($( $name:ident ),+ $( , )?) => {
                    match type_id {
                        $(
                            BuiltInTypeId::$name => Self::$name($name::read(data)),
                        )+
                        _ => panic!(),
                    }
                };
            }
            read!(
                Boolean,
                SByte,
                Byte,
                Int16,
                UInt16,
                Int32,
                UInt32,
                Int64,
                UInt64,
                Float,
                Double,
                String,
                // DateTime,
                Guid,
                ByteString,
                XmlElement,
                NodeId,
                // ExpandedNodeId,
                // StatusCode,
                QualifiedName,
                LocalizedText,
                ExtensionObject,
                // DataValue,
                // // Variant,
                // DiagnosticInfo,
            )
        }
        fn type_id(&self) -> BuiltInTypeId {
            macro_rules! type_id {
                ($( $name:ident ),+ $( , )?) => {
                    match self {
                        $(
                            Self::$name(_) => BuiltInTypeId::$name,
                        )+
                    }
                };
            }
            type_id!(
                Boolean,
                SByte,
                Byte,
                Int16,
                UInt16,
                Int32,
                UInt32,
                Int64,
                UInt64,
                Float,
                Double,
                String,
                // DateTime,
                Guid,
                ByteString,
                XmlElement,
                NodeId,
                // ExpandedNodeId,
                // StatusCode,
                QualifiedName,
                LocalizedText,
                ExtensionObject,
                // DataValue,
                // // Variant,
                // DiagnosticInfo,
            )
        }
    }

    // [5.1.2 Built-in Types](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.2)
    enum VariantArray {
        Boolean(Vec<Boolean>),
        SByte(Vec<SByte>),
        Byte(Vec<Byte>),
        Int16(Vec<Int16>),
        UInt16(Vec<UInt16>),
        Int32(Vec<Int32>),
        UInt32(Vec<UInt32>),
        Int64(Vec<Int64>),
        UInt64(Vec<UInt64>),
        Float(Vec<Float>),
        Double(Vec<Double>),
        String(Vec<String>),
        // DateTime(Vec<DateTime>),
        Guid(Vec<Guid>),
        ByteString(Vec<ByteString>),
        XmlElement(Vec<XmlElement>),
        NodeId(Vec<NodeId>),
        // ExpandedNodeId(Vec<ExpandedNodeId>),
        // StatusCode(Vec<StatusCode>),
        QualifiedName(Vec<QualifiedName>),
        LocalizedText(Vec<LocalizedText>),
        ExtensionObject(Vec<ExtensionObject>),
        // DataValue(Vec<DataValue>),
        Variant(Vec<Variant>),
        // // DiagnosticInfo(Vec<DiagnosticInfo>),
    }
    impl VariantArray {
        fn read(type_id: BuiltInTypeId, length: usize, data: &mut Bytes) -> Self {
            macro_rules! read {
                ($( $name:ident ),+ $( , )?) => {
                    match type_id {
                        $(
                            BuiltInTypeId::$name => {
                                Self::$name((0..length).map(|_| $name::read(data)).collect())
                            },
                        )+
                        _ => panic!(),
                    }
                };
            }
            read!(
                Boolean,
                SByte,
                Byte,
                Int16,
                UInt16,
                Int32,
                UInt32,
                Int64,
                UInt64,
                Float,
                Double,
                String,
                // DateTime,
                Guid,
                ByteString,
                XmlElement,
                NodeId,
                // ExpandedNodeId,
                // StatusCode,
                QualifiedName,
                LocalizedText,
                ExtensionObject,
                // DataValue,
                Variant,
                // DiagnosticInfo,
            )
        }
        fn type_id(&self) -> BuiltInTypeId {
            macro_rules! type_id {
                ($( $name:ident ),+ $( , )?) => {
                    match self {
                        $(
                            Self::$name(_) => BuiltInTypeId::$name,
                        )+
                    }
                };
            }
            type_id!(
                Boolean,
                SByte,
                Byte,
                Int16,
                UInt16,
                Int32,
                UInt32,
                Int64,
                UInt64,
                Float,
                Double,
                String,
                // DateTime,
                Guid,
                ByteString,
                XmlElement,
                NodeId,
                // ExpandedNodeId,
                // StatusCode,
                QualifiedName,
                LocalizedText,
                ExtensionObject,
                // DataValue,
                Variant,
                // DiagnosticInfo,
            )
        }
        fn len(&self) -> usize {
            macro_rules! len {
                ($( $name:ident ),+ $( , )?) => {
                    match self {
                        $(
                            Self::$name(values) => values.len(),
                        )+
                    }
                };
            }
            len!(
                Boolean,
                SByte,
                Byte,
                Int16,
                UInt16,
                Int32,
                UInt32,
                Int64,
                UInt64,
                Float,
                Double,
                String,
                // DateTime,
                Guid,
                ByteString,
                XmlElement,
                NodeId,
                // ExpandedNodeId,
                // StatusCode,
                QualifiedName,
                LocalizedText,
                ExtensionObject,
                // DataValue,
                Variant,
                // DiagnosticInfo,
            )
        }
    }

    pub struct Array<T>(Option<Vec<T>>);
    impl<T> Binary for Array<T>
    where
        T: Binary,
    {
        fn read(data: &mut Bytes) -> Self {
            let length = Int32::read(data);
            if length.0 == -1 {
                return Self(None);
            }
            let length = usize::try_from(length.0).unwrap();
            Self(Some((0..length).map(|_| T::read(data)).collect()))
        }
    }

    trait BinaryReader {
        type Value;
        fn binary_read(&mut self, data: &mut Bytes) -> Self::Value;
    }
    struct ArrayReader<T>(T);
    impl<T> BinaryReader for ArrayReader<T>
    where
        T: BinaryReader,
    {
        type Value = Array<T::Value>;
        fn binary_read(&mut self, data: &mut Bytes) -> Self::Value {
            let length = Int32::read(data);
            if length.0 == -1 {
                return Array(None);
            }
            let length = usize::try_from(length.0).unwrap();
            Array(Some(
                (0..length).map(|_| self.0.binary_read(data)).collect(),
            ))
        }
    }

    // [8.33 UInteger](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.33)
    pub enum UInteger {
        Byte(Byte),
        UInt16(UInt16),
        UInt32(UInt32),
        UInt64(UInt64),
    }
}
