use crate::{ua, DataType};

#[allow(dead_code, reason = "--no-default-features")]
pub(crate) trait ServiceRequest: DataType + 'static {
    type Response: ServiceResponse;
}

#[allow(dead_code, reason = "--no-default-features")]
pub(crate) trait ServiceResponse: DataType + 'static {
    type Request: ServiceRequest;

    fn service_result(&self) -> ua::StatusCode;
}
