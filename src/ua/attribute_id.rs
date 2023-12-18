use std::fmt;

use open62541_sys::UA_AttributeId;

/// Wrapper for attribute IDs from [`open62541_sys`].
#[derive(Clone, Debug)]
pub struct AttributeId(UA_AttributeId);

impl AttributeId {
    #[must_use]
    pub const fn value() -> Self {
        Self(UA_AttributeId::UA_ATTRIBUTEID_VALUE)
    }

    /// Creates wrapper by taking ownership of `src`.
    #[allow(dead_code)]
    #[must_use]
    pub(crate) const fn new(src: UA_AttributeId) -> Self {
        Self(src)
    }

    /// Gives up ownership and returns inner value.
    #[must_use]
    pub(crate) const fn into_inner(self) -> UA_AttributeId {
        self.0
    }

    #[allow(clippy::unnecessary_cast)]
    #[must_use]
    pub(crate) const fn as_u32(&self) -> u32 {
        // This cast is necessary on Windows builds where `UA_BrowseResultMask` wraps an `i32`.
        (self.0).0 as u32
    }
}

impl fmt::Display for AttributeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self.0 {
            UA_AttributeId::UA_ATTRIBUTEID_NODEID => "NODEID",
            UA_AttributeId::UA_ATTRIBUTEID_NODECLASS => "NODECLASS",
            UA_AttributeId::UA_ATTRIBUTEID_BROWSENAME => "BROWSENAME",
            UA_AttributeId::UA_ATTRIBUTEID_DISPLAYNAME => "DISPLAYNAME",
            UA_AttributeId::UA_ATTRIBUTEID_DESCRIPTION => "DESCRIPTION",
            UA_AttributeId::UA_ATTRIBUTEID_WRITEMASK => "WRITEMASK",
            UA_AttributeId::UA_ATTRIBUTEID_USERWRITEMASK => "USERWRITEMASK",
            UA_AttributeId::UA_ATTRIBUTEID_ISABSTRACT => "ISABSTRACT",
            UA_AttributeId::UA_ATTRIBUTEID_SYMMETRIC => "SYMMETRIC",
            UA_AttributeId::UA_ATTRIBUTEID_INVERSENAME => "INVERSENAME",
            UA_AttributeId::UA_ATTRIBUTEID_CONTAINSNOLOOPS => "CONTAINSNOLOOPS",
            UA_AttributeId::UA_ATTRIBUTEID_EVENTNOTIFIER => "EVENTNOTIFIER",
            UA_AttributeId::UA_ATTRIBUTEID_VALUE => "VALUE",
            UA_AttributeId::UA_ATTRIBUTEID_DATATYPE => "DATATYPE",
            UA_AttributeId::UA_ATTRIBUTEID_VALUERANK => "VALUERANK",
            UA_AttributeId::UA_ATTRIBUTEID_ARRAYDIMENSIONS => "ARRAYDIMENSIONS",
            UA_AttributeId::UA_ATTRIBUTEID_ACCESSLEVEL => "ACCESSLEVEL",
            UA_AttributeId::UA_ATTRIBUTEID_USERACCESSLEVEL => "USERACCESSLEVEL",
            UA_AttributeId::UA_ATTRIBUTEID_MINIMUMSAMPLINGINTERVAL => "MINIMUMSAMPLINGINTERVAL",
            UA_AttributeId::UA_ATTRIBUTEID_HISTORIZING => "HISTORIZING",
            UA_AttributeId::UA_ATTRIBUTEID_EXECUTABLE => "EXECUTABLE",
            UA_AttributeId::UA_ATTRIBUTEID_USEREXECUTABLE => "USEREXECUTABLE",
            UA_AttributeId::UA_ATTRIBUTEID_DATATYPEDEFINITION => "DATATYPEDEFINITION",
            UA_AttributeId::UA_ATTRIBUTEID_ROLEPERMISSIONS => "ROLEPERMISSIONS",
            UA_AttributeId::UA_ATTRIBUTEID_USERROLEPERMISSIONS => "USERROLEPERMISSIONS",
            UA_AttributeId::UA_ATTRIBUTEID_ACCESSRESTRICTIONS => "ACCESSRESTRICTIONS",
            UA_AttributeId::UA_ATTRIBUTEID_ACCESSLEVELEX => "ACCESSLEVELEX",
            _ => "?",
        };
        f.write_str(str)
    }
}
