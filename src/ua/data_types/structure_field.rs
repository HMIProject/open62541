use crate::ua;

crate::data_type!(StructureField);
crate::member_accessors!(StructureField {
    name: &ua::String,
    description: &ua::LocalizedText,
    dataType: &ua::NodeId,
    valueRank: i32,
    arrayDimensions: [ua::UInt32],
    maxStringLength: u32,
    isOptional: bool
});
