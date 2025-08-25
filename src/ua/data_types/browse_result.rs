use crate::{ua, DataType as _, Error};

crate::data_type!(BrowseResult);

impl BrowseResult {
    #[must_use]
    pub const fn status_code(&self) -> ua::StatusCode {
        ua::StatusCode::new(self.0.statusCode)
    }

    #[must_use]
    pub fn references(&self) -> Option<ua::Array<ua::ReferenceDescription>> {
        // TODO: Adjust signature to return non-owned value instead.
        ua::Array::from_raw_parts(self.0.referencesSize, self.0.references)
    }

    /// Gets continuation point.
    ///
    /// Browse results include a continuation point when not all references could be returned. Pass
    /// it to [`AsyncClient::browse_next()`] to request the remaining references.
    ///
    /// [`AsyncClient::browse_next()`]: crate::AsyncClient::browse_next
    #[must_use]
    pub fn continuation_point(&self) -> Option<ua::ContinuationPoint> {
        ua::ContinuationPoint::new(ua::ByteString::raw_ref(&self.0.continuationPoint).clone())
    }

    /// Evaluates this instance and converts it into the corresponding result type.
    pub fn eval(&self, node_id: Option<&ua::NodeId>) -> crate::BrowseResult {
        // Make sure to verify the inner status code inside `BrowseResult`. The service request finishes
        // without error, even when browsing the node has failed.
        Error::verify_good(&self.status_code())?;

        let references = if let Some(references) = self.references() {
            references.into_vec()
        } else {
            // When no references exist, some OPC UA servers do not return an empty references array but
            // an invalid (unset) one instead, e.g. Siemens SIMOTION. We treat it as an empty array, and
            // continue without error.
            if let Some(node_id) = node_id {
                log::debug!("Browsing {node_id} returned unset references, assuming none exist");
            } else {
                log::debug!(
                    "Browsing continuation point returned unset references, assuming none exist",
                );
            }
            Vec::new()
        };

        Ok((references, self.continuation_point()))
    }
}
