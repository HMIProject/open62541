use crate::{ua, DataType as _, ServiceRequest};

crate::data_type!(BrowseNextRequest);

impl BrowseNextRequest {
    #[must_use]
    pub fn with_continuation_points(
        mut self,
        continuation_points: &[ua::ContinuationPoint],
    ) -> Self {
        let array = ua::Array::from_iter(
            continuation_points
                .iter()
                .map(ua::ContinuationPoint::to_byte_string),
        );
        array.move_into_raw(
            &mut self.0.continuationPointsSize,
            &mut self.0.continuationPoints,
        );
        self
    }

    #[must_use]
    pub const fn with_release_continuation_points(
        mut self,
        release_continuation_points: bool,
    ) -> Self {
        self.0.releaseContinuationPoints = release_continuation_points;
        self
    }
}

impl ServiceRequest for BrowseNextRequest {
    type Response = ua::BrowseNextResponse;

    fn request_header(&self) -> &ua::RequestHeader {
        ua::RequestHeader::raw_ref(&self.0.requestHeader)
    }

    fn request_header_mut(&mut self) -> &mut ua::RequestHeader {
        ua::RequestHeader::raw_mut(&mut self.0.requestHeader)
    }
}
