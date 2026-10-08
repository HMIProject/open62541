use crate::{DataType as _, ServiceRequest, UaIterExt as _, ua};

crate::data_type!(BrowseNextRequest);
crate::member_accessors!(BrowseNextRequest {
    continuationPoints: [ua::ContinuationPoint]
});

impl BrowseNextRequest {
    #[must_use]
    pub fn with_continuation_points(
        mut self,
        continuation_points: &[ua::ContinuationPoint],
    ) -> Self {
        continuation_points
            .iter()
            .map(ua::ContinuationPoint::to_byte_string)
            .collect_ua_array()
            .move_into_raw(
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
