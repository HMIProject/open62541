use open62541_sys::UA_NS0ID_HIERARCHICALREFERENCES;

use crate::{DataType, ua};

crate::data_type!(BrowseDescription);
crate::member_accessors!(BrowseDescription {
    nodeId: &ua::NodeId,
    referenceTypeId: &ua::NodeId,
    includeSubtypes: bool,
    #[from_inner]
    nodeClassMask: ua::NodeClassMask,
    #[from_inner]
    resultMask: ua::BrowseResultMask,
    #[skip(uses = BrowseDescription::browse_direction)]
    browseDirection: ua::BrowseDirection
});

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

    #[must_use]
    pub const fn browse_direction(&self) -> ua::BrowseDirection {
        // this is cursed
        ua::BrowseDirection(open62541_sys::UA_BrowseDirection(self.0.browseDirection.0))
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
