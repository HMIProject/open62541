use crate::ua;

crate::data_type!(
    ReferenceDescription,
    UA_ReferenceDescription,
    UA_TYPES_REFERENCEDESCRIPTION
);

impl ReferenceDescription {
    #[must_use]
    pub fn reference_type_id(&self) -> ua::NodeId {
        ua::NodeId::from_ref(&self.0.referenceTypeId)
    }

    #[must_use]
    pub const fn is_forward(&self) -> bool {
        self.0.isForward
    }

    #[must_use]
    pub fn node_id(&self) -> ua::ExpandedNodeId {
        ua::ExpandedNodeId::from_ref(&self.0.nodeId)
    }

    #[must_use]
    pub fn browse_name(&self) -> ua::QualifiedName {
        ua::QualifiedName::from_ref(&self.0.browseName)
    }

    #[must_use]
    pub fn display_name(&self) -> ua::LocalizedText {
        ua::LocalizedText::from_ref(&self.0.displayName)
    }

    #[must_use]
    pub const fn node_class(&self) -> ua::NodeClass {
        ua::NodeClass::new(self.0.nodeClass)
    }

    #[must_use]
    pub fn type_definition(&self) -> ua::ExpandedNodeId {
        ua::ExpandedNodeId::from_ref(&self.0.typeDefinition)
    }
}
