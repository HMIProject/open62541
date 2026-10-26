use crate::ua;

crate::data_type!(BuildInfo);
crate::member_accessors!(BuildInfo {
    #[from_inner]
    buildDate: ua::DateTime,
    buildNumber: &ua::String,
    manufacturerName: &ua::String,
    productName: &ua::String,
    productUri: &ua::String,
    softwareVersion: &ua::String
});
