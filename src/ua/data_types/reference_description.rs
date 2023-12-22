use crate::{ua, DataType as _};

crate::data_type!(ReferenceDescription);

impl ReferenceDescription {
    #[must_use]
    pub fn reference_type_id(&self) -> ua::NodeId {
        ua::NodeId::clone_raw(&self.0.referenceTypeId)
    }

    #[must_use]
    pub const fn is_forward(&self) -> bool {
        self.0.isForward
    }

    #[must_use]
    pub fn node_id(&self) -> ua::ExpandedNodeId {
        ua::ExpandedNodeId::clone_raw(&self.0.nodeId)
    }

    #[must_use]
    pub fn browse_name(&self) -> ua::QualifiedName {
        ua::QualifiedName::clone_raw(&self.0.browseName)
    }

    #[must_use]
    pub fn display_name(&self) -> ua::LocalizedText {
        ua::LocalizedText::clone_raw(&self.0.displayName)
    }

    #[must_use]
    pub fn node_class(&self) -> ua::NodeClass {
        ua::NodeClass::new(self.0.nodeClass.clone())
    }

    #[must_use]
    pub fn type_definition(&self) -> ua::ExpandedNodeId {
        ua::ExpandedNodeId::clone_raw(&self.0.typeDefinition)
    }
}
