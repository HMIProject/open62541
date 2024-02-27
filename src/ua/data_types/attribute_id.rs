use std::hash;

use open62541_sys::UA_AttributeId;

crate::data_type!(AttributeId, UInt32);

crate::enum_variants!(
    AttributeId,
    UA_AttributeId,
    [
        NODEID,
        NODECLASS,
        BROWSENAME,
        DISPLAYNAME,
        DESCRIPTION,
        WRITEMASK,
        USERWRITEMASK,
        ISABSTRACT,
        SYMMETRIC,
        INVERSENAME,
        CONTAINSNOLOOPS,
        EVENTNOTIFIER,
        VALUE,
        DATATYPE,
        VALUERANK,
        ARRAYDIMENSIONS,
        ACCESSLEVEL,
        USERACCESSLEVEL,
        MINIMUMSAMPLINGINTERVAL,
        HISTORIZING,
        EXECUTABLE,
        USEREXECUTABLE,
        DATATYPEDEFINITION,
        ROLEPERMISSIONS,
        USERROLEPERMISSIONS,
        ACCESSRESTRICTIONS,
        ACCESSLEVELEX,
    ]
);

impl AttributeId {
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
