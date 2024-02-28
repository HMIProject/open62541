crate::data_type!(NodeClass);

crate::enum_variants!(
    NodeClass,
    UA_NodeClass,
    [
        UNSPECIFIED,
        OBJECT,
        VARIABLE,
        METHOD,
        OBJECTTYPE,
        VARIABLETYPE,
        REFERENCETYPE,
        DATATYPE,
        VIEW,
    ],
);
