use open62541_sys::UA_NodeClass;

use crate::ua;

crate::data_type!(ReferenceDescription);
crate::member_accessors!(ReferenceDescription {
    referenceTypeId: &ua::NodeId,
    isForward: bool,
    nodeId: &ua::ExpandedNodeId,
    browseName: &ua::QualifiedName,
    displayName: &ua::LocalizedText,
    typeDefinition: &ua::ExpandedNodeId,
    #[enum(UA_NodeClass)]
    nodeClass: ua::NodeClass
});
