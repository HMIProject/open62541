crate::data_type!(StructureType);

crate::enum_variants!(
    StructureType,
    UA_StructureType,
    [
        STRUCTURE,
        STRUCTUREWITHOPTIONALFIELDS,
        UNION,
        STRUCTUREWITHSUBTYPEDVALUES,
        UNIONWITHSUBTYPEDVALUES,
    ],
);
