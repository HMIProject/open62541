use open62541_sys::UA_CreateSubscriptionRequest_default;

crate::data_type!(
    CreateSubscriptionRequest,
    UA_CreateSubscriptionRequest,
    UA_TYPES_CREATESUBSCRIPTIONREQUEST
);

impl Default for CreateSubscriptionRequest {
    fn default() -> Self {
        let inner = unsafe { UA_CreateSubscriptionRequest_default() };
        Self(inner)
    }
}
