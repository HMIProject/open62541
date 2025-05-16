use crate::{ua, DataType};

#[cfg_attr(not(feature = "tokio"), expect(dead_code, reason = "unused"))]
pub(crate) trait ServiceRequest: DataType + 'static {
    type Response: ServiceResponse;
}

#[cfg_attr(not(feature = "tokio"), expect(dead_code, reason = "unused"))]
pub(crate) trait ServiceResponse: DataType + 'static {
    type Request: ServiceRequest;

    fn service_result(&self) -> ua::StatusCode;
}
