use crate::ua;

crate::data_type!(BrowsePathResult);

crate::member_accessors!(BrowsePathResult {
    targets: [ua::BrowsePathTarget],
    #[from_inner]
    statusCode: ua::StatusCode
});
