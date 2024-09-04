use crate::{ua, Result};

/// Result type for browsing.
pub type BrowseResult = Result<(Vec<ua::ReferenceDescription>, Option<ua::ContinuationPoint>)>;
