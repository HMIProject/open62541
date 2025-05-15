use open62541_sys::UA_NS0ID_HIERARCHICALREFERENCES;

use crate::{ua, DataType};

crate::data_type!(BrowseDescription);

impl BrowseDescription {
    #[must_use]
    pub fn with_node_id(mut self, node_id: &ua::NodeId) -> Self {
        node_id.clone_into_raw(&mut self.0.nodeId);
        self
    }

    #[must_use]
    pub fn with_browse_direction(mut self, browse_direction: &ua::BrowseDirection) -> Self {
        browse_direction.clone_into_raw(&mut self.0.browseDirection);
        self
    }

    #[must_use]
    pub fn with_reference_type_id(mut self, reference_type_id: &ua::NodeId) -> Self {
        reference_type_id.clone_into_raw(&mut self.0.referenceTypeId);
        self
    }

    #[must_use]
    pub const fn with_include_subtypes(mut self, include_subtypes: bool) -> Self {
        self.0.includeSubtypes = include_subtypes;
        self
    }

    #[must_use]
    pub const fn with_node_class_mask(mut self, node_class_mask: &ua::NodeClassMask) -> Self {
        self.0.nodeClassMask = node_class_mask.as_u32();
        self
    }

    #[must_use]
    pub const fn with_result_mask(mut self, result_mask: &ua::BrowseResultMask) -> Self {
        self.0.resultMask = result_mask.as_u32();
        self
    }

    #[allow(dead_code, reason = "--no-default-features")]
    #[must_use]
    pub(crate) fn node_id(&self) -> &ua::NodeId {
        ua::NodeId::raw_ref(&self.0.nodeId)
    }
}

impl Default for BrowseDescription {
    fn default() -> Self {
        Self::init()
            .with_browse_direction(&ua::BrowseDirection::FORWARD)
            .with_reference_type_id(&ua::NodeId::numeric(0, UA_NS0ID_HIERARCHICALREFERENCES))
            .with_include_subtypes(true)
            .with_result_mask(&ua::BrowseResultMask::ALL)
    }
}
