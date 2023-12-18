use std::fmt;

use open62541_sys::UA_NodeClass;

/// Wrapper for node class from [`open62541_sys`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeClass(UA_NodeClass);

impl NodeClass {
    /// Creates wrapper by taking ownership of `src`.
    #[must_use]
    pub(crate) const fn new(src: UA_NodeClass) -> Self {
        Self(src)
    }

    /// Gives up ownership and returns inner value.
    #[allow(dead_code)]
    #[must_use]
    pub(crate) const fn into_inner(self) -> UA_NodeClass {
        self.0
    }
}

impl fmt::Display for NodeClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self.0 {
            UA_NodeClass::UA_NODECLASS_UNSPECIFIED => "UNSPECIFIED",
            UA_NodeClass::UA_NODECLASS_OBJECT => "OBJECT",
            UA_NodeClass::UA_NODECLASS_VARIABLE => "VARIABLE",
            UA_NodeClass::UA_NODECLASS_METHOD => "METHOD",
            UA_NodeClass::UA_NODECLASS_OBJECTTYPE => "OBJECTTYPE",
            UA_NodeClass::UA_NODECLASS_VARIABLETYPE => "VARIABLETYPE",
            UA_NodeClass::UA_NODECLASS_REFERENCETYPE => "REFERENCETYPE",
            UA_NodeClass::UA_NODECLASS_DATATYPE => "DATATYPE",
            UA_NodeClass::UA_NODECLASS_VIEW => "VIEW",
            _ => "?",
        };
        f.write_str(str)
    }
}
