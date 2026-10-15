use crate::ua;

crate::data_type!(ReferenceDescription);
crate::member_accessors!(ReferenceDescription {
    referenceTypeId: &ua::NodeId,
    isForward: bool,
    nodeId: &ua::ExpandedNodeId,
    browseName: &ua::QualifiedName,
    displayName: &ua::LocalizedText,
    typeDefinition: &ua::ExpandedNodeId,
    #[enum]
    nodeClass: ua::NodeClass
});
