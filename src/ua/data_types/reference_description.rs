use crate::ua;

crate::data_type!(
    ReferenceDescription,
    UA_ReferenceDescription,
    UA_TYPES_REFERENCEDESCRIPTION
);

impl ReferenceDescription {
    #[must_use]
    pub const fn reference_type_id(&self) -> ua::NodeId {
        ua::NodeId::new(self.0.referenceTypeId)
    }

    #[must_use]
    pub const fn is_forward(&self) -> bool {
        self.0.isForward
    }

    #[must_use]
    pub const fn node_id(&self) -> ua::ExpandedNodeId {
        ua::ExpandedNodeId::new(self.0.nodeId)
    }

    #[must_use]
    pub const fn browse_name(&self) -> ua::QualifiedName {
        ua::QualifiedName::new(self.0.browseName)
    }

    #[must_use]
    pub const fn display_name(&self) -> ua::LocalizedText {
        ua::LocalizedText::new(self.0.displayName)
    }

    #[must_use]
    pub const fn node_class(&self) -> ua::NodeClass {
        ua::NodeClass::new(self.0.nodeClass)
    }

    #[must_use]
    pub const fn type_definition(&self) -> ua::ExpandedNodeId {
        ua::ExpandedNodeId::new(self.0.typeDefinition)
    }
}
