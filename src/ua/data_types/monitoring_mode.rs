crate::data_type!(MonitoringMode);

crate::enum_variants!(
    MonitoringMode,
    UA_MonitoringMode,
    [DISABLED, SAMPLING, REPORTING]
);
