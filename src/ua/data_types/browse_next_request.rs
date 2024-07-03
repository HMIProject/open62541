use crate::{ua, ServiceRequest};

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
    pub fn with_release_continuation_points(mut self, release_continuation_points: bool) -> Self {
        self.0.releaseContinuationPoints = release_continuation_points;
        self
    }
}

impl ServiceRequest for BrowseNextRequest {
    type Response = ua::BrowseNextResponse;
}
