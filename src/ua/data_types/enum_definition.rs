use crate::ua;

crate::data_type!(EnumDefinition);
crate::member_accessors!(EnumDefinition {
    fields: [ua::EnumField]
});
