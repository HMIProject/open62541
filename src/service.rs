use crate::{DataType, ua};

/// A generic service request.
///
/// Defines methods supported by all service request types.
pub trait ServiceRequest: DataType + 'static {
    type Response: ServiceResponse;

    /// Returns the request header.
    #[must_use]
    fn request_header(&self) -> &ua::RequestHeader;

    /// Returns the mutable request header.
    #[must_use]
    fn request_header_mut(&mut self) -> &mut ua::RequestHeader;
}

/// A generic service response.
///
/// Defines methods supported by all service response types.
pub trait ServiceResponse: DataType + 'static {
    type Request: ServiceRequest;

    /// Returns the response header.
    #[must_use]
    fn response_header(&self) -> &ua::ResponseHeader;
}
