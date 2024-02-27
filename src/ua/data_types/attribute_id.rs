use std::{fmt, hash};

use open62541_sys::UA_AttributeId;

crate::data_type!(AttributeId, UInt32);

impl AttributeId {
    pub const NODEID: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_NODEID);
    pub const NODECLASS: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_NODECLASS);
    pub const BROWSENAME: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_BROWSENAME);
    pub const DISPLAYNAME: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_DISPLAYNAME);
    pub const DESCRIPTION: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_DESCRIPTION);
    pub const WRITEMASK: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_WRITEMASK);
    pub const USERWRITEMASK: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_USERWRITEMASK);
    pub const ISABSTRACT: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_ISABSTRACT);
    pub const SYMMETRIC: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_SYMMETRIC);
    pub const INVERSENAME: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_INVERSENAME);
    pub const CONTAINSNOLOOPS: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_CONTAINSNOLOOPS);
    pub const EVENTNOTIFIER: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_EVENTNOTIFIER);
    pub const VALUE: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_VALUE);
    pub const DATATYPE: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_DATATYPE);
    pub const VALUERANK: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_VALUERANK);
    pub const ARRAYDIMENSIONS: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_ARRAYDIMENSIONS);
    pub const ACCESSLEVEL: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_ACCESSLEVEL);
    pub const USERACCESSLEVEL: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_USERACCESSLEVEL);
    pub const MINIMUMSAMPLINGINTERVAL: Self =
        Self(UA_AttributeId::UA_ATTRIBUTEID_MINIMUMSAMPLINGINTERVAL);
    pub const HISTORIZING: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_HISTORIZING);
    pub const EXECUTABLE: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_EXECUTABLE);
    pub const USEREXECUTABLE: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_USEREXECUTABLE);
    pub const DATATYPEDEFINITION: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_DATATYPEDEFINITION);
    pub const ROLEPERMISSIONS: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_ROLEPERMISSIONS);
    pub const USERROLEPERMISSIONS: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_USERROLEPERMISSIONS);
    pub const ACCESSRESTRICTIONS: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_ACCESSRESTRICTIONS);
    pub const ACCESSLEVELEX: Self = Self(UA_AttributeId::UA_ATTRIBUTEID_ACCESSLEVELEX);

    #[deprecated(note = "use `Self::VALUE` instead")]
    #[must_use]
    pub const fn value() -> Self {
        Self(UA_AttributeId::UA_ATTRIBUTEID_VALUE)
    }

    pub(crate) fn as_u32(&self) -> u32 {
        // This cast is necessary on Windows builds with inner type `i32`.
        #[allow(clippy::useless_conversion)]
        u32::try_from((self.0).0).expect("should convert to u32")
    }
}

impl hash::Hash for AttributeId {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
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
            _ => return write!(f, "{}", self.as_u32()),
        };
        f.write_str(str)
    }
}
