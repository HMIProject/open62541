// We do not expose the inner enum. We want to use a proper `u32` for bit operations on the mask and
// we want to be clear about what is an initial (const, enum-like) value and what is a derived mask;
// specifically, the bitmask type is _not_ an enum even though declared so in `open62541-sys`.
mod inner {
    crate::data_type!(NodeAttributesMask);

    crate::enum_variants!(
        NodeAttributesMask,
        UA_NodeAttributesMask,
        [
            NONE,
            ACCESSLEVEL,
            ARRAYDIMENSIONS,
            BROWSENAME,
            CONTAINSNOLOOPS,
            DATATYPE,
            DESCRIPTION,
            DISPLAYNAME,
            EVENTNOTIFIER,
            EXECUTABLE,
            HISTORIZING,
            INVERSENAME,
            ISABSTRACT,
            MINIMUMSAMPLINGINTERVAL,
            NODECLASS,
            NODEID,
            SYMMETRIC,
            USERACCESSLEVEL,
            USEREXECUTABLE,
            USERWRITEMASK,
            VALUERANK,
            WRITEMASK,
            VALUE,
            DATATYPEDEFINITION,
            ROLEPERMISSIONS,
            ACCESSRESTRICTIONS,
            ALL,
            BASENODE,
            OBJECT,
            OBJECTTYPE,
            VARIABLE,
            VARIABLETYPE,
            METHOD,
            REFERENCETYPE,
            VIEW,
        ],
    );
}

/// Wrapper for node attributes mask from [`open62541_sys`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeAttributesMask(u32);

crate::bitmask_ops!(NodeAttributesMask);

impl NodeAttributesMask {
    pub const NONE: Self = Self(inner::NodeAttributesMask::NONE_U32);
    pub const ACCESSLEVEL: Self = Self(inner::NodeAttributesMask::ACCESSLEVEL_U32);
    pub const ARRAYDIMENSIONS: Self = Self(inner::NodeAttributesMask::ARRAYDIMENSIONS_U32);
    pub const BROWSENAME: Self = Self(inner::NodeAttributesMask::BROWSENAME_U32);
    pub const CONTAINSNOLOOPS: Self = Self(inner::NodeAttributesMask::CONTAINSNOLOOPS_U32);
    pub const DATATYPE: Self = Self(inner::NodeAttributesMask::DATATYPE_U32);
    pub const DESCRIPTION: Self = Self(inner::NodeAttributesMask::DESCRIPTION_U32);
    pub const DISPLAYNAME: Self = Self(inner::NodeAttributesMask::DISPLAYNAME_U32);
    pub const EVENTNOTIFIER: Self = Self(inner::NodeAttributesMask::EVENTNOTIFIER_U32);
    pub const EXECUTABLE: Self = Self(inner::NodeAttributesMask::EXECUTABLE_U32);
    pub const HISTORIZING: Self = Self(inner::NodeAttributesMask::HISTORIZING_U32);
    pub const INVERSENAME: Self = Self(inner::NodeAttributesMask::INVERSENAME_U32);
    pub const ISABSTRACT: Self = Self(inner::NodeAttributesMask::ISABSTRACT_U32);
    pub const MINIMUMSAMPLINGINTERVAL: Self =
        Self(inner::NodeAttributesMask::MINIMUMSAMPLINGINTERVAL_U32);
    pub const NODECLASS: Self = Self(inner::NodeAttributesMask::NODECLASS_U32);
    pub const NODEID: Self = Self(inner::NodeAttributesMask::NODEID_U32);
    pub const SYMMETRIC: Self = Self(inner::NodeAttributesMask::SYMMETRIC_U32);
    pub const USERACCESSLEVEL: Self = Self(inner::NodeAttributesMask::USERACCESSLEVEL_U32);
    pub const USEREXECUTABLE: Self = Self(inner::NodeAttributesMask::USEREXECUTABLE_U32);
    pub const USERWRITEMASK: Self = Self(inner::NodeAttributesMask::USERWRITEMASK_U32);
    pub const VALUERANK: Self = Self(inner::NodeAttributesMask::VALUERANK_U32);
    pub const WRITEMASK: Self = Self(inner::NodeAttributesMask::WRITEMASK_U32);
    pub const VALUE: Self = Self(inner::NodeAttributesMask::VALUE_U32);
    pub const DATATYPEDEFINITION: Self = Self(inner::NodeAttributesMask::DATATYPEDEFINITION_U32);
    pub const ROLEPERMISSIONS: Self = Self(inner::NodeAttributesMask::ROLEPERMISSIONS_U32);
    pub const ACCESSRESTRICTIONS: Self = Self(inner::NodeAttributesMask::ACCESSRESTRICTIONS_U32);
    pub const ALL: Self = Self(inner::NodeAttributesMask::ALL_U32);
    pub const BASENODE: Self = Self(inner::NodeAttributesMask::BASENODE_U32);
    pub const OBJECT: Self = Self(inner::NodeAttributesMask::OBJECT_U32);
    pub const OBJECTTYPE: Self = Self(inner::NodeAttributesMask::OBJECTTYPE_U32);
    pub const VARIABLE: Self = Self(inner::NodeAttributesMask::VARIABLE_U32);
    pub const VARIABLETYPE: Self = Self(inner::NodeAttributesMask::VARIABLETYPE_U32);
    pub const METHOD: Self = Self(inner::NodeAttributesMask::METHOD_U32);
    pub const REFERENCETYPE: Self = Self(inner::NodeAttributesMask::REFERENCETYPE_U32);
    pub const VIEW: Self = Self(inner::NodeAttributesMask::VIEW_U32);

    pub(crate) const fn from_u32(mask: u32) -> Self {
        Self(mask)
    }

    pub(crate) const fn as_u32(&self) -> u32 {
        self.0
    }
}
