use crate::ua;

/// Wrapper for node class mask from [`open62541_sys`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeClassMask(u32);

crate::bitmask_ops!(NodeClassMask);

impl NodeClassMask {
    pub const OBJECT: Self = Self(ua::NodeClass::OBJECT_U32);
    pub const VARIABLE: Self = Self(ua::NodeClass::VARIABLE_U32);
    pub const METHOD: Self = Self(ua::NodeClass::METHOD_U32);
    pub const OBJECTTYPE: Self = Self(ua::NodeClass::OBJECTTYPE_U32);
    pub const VARIABLETYPE: Self = Self(ua::NodeClass::VARIABLETYPE_U32);
    pub const REFERENCETYPE: Self = Self(ua::NodeClass::REFERENCETYPE_U32);
    pub const DATATYPE: Self = Self(ua::NodeClass::DATATYPE_U32);
    pub const VIEW: Self = Self(ua::NodeClass::VIEW_U32);

    pub(crate) const fn from_u32(mask: u32) -> Self {
        Self(mask)
    }

    pub(crate) const fn as_u32(&self) -> u32 {
        self.0
    }
}
