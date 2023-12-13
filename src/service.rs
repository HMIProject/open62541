use crate::{ua, DataType};

pub(crate) trait ServiceRequest: DataType + 'static {
    type Response: ServiceResponse;
}

pub(crate) trait ServiceResponse: DataType + 'static {
    type Request: ServiceRequest;

    fn service_result(&self) -> ua::StatusCode;
}
