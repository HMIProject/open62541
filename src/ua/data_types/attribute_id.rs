use std::hash;

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
    ],
);

impl hash::Hash for AttributeId {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
