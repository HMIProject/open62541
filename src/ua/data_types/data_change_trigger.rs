crate::data_type!(DataChangeTrigger, UInt32);

crate::enum_variants!(
    DataChangeTrigger,
    UA_DataChangeTrigger,
    [STATUS, STATUSVALUE, STATUSVALUETIMESTAMP,]
);
