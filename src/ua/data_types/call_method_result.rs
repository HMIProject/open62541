use crate::ua;

crate::data_type!(CallMethodResult);
crate::member_accessors!(CallMethodResult {
    #[from_inner]
    statusCode: ua::StatusCode,
    inputArgumentResults: [ua::StatusCode],
    outputArguments: [ua::Variant]
});
