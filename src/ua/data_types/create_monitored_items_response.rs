use crate::ua;

crate::data_type!(CreateMonitoredItemsResponse);
crate::member_accessors!(CreateMonitoredItemsResponse {
    results: [ua::MonitoredItemCreateResult],
    diagnosticInfos: [ua::DiagnosticInfo],
    responseHeader: &ua::ResponseHeader
});
