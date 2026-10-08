use crate::ua;

crate::data_type!(EnumField);
crate::member_accessors!(EnumField {
    description: &ua::LocalizedText,
    displayName: &ua::LocalizedText,
    name: &ua::String,
    value: i64
});
