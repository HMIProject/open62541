use crate::{MonitoringFilter, ua};

crate::data_type!(AggregateFilter);
crate::member_accessors!(AggregateFilter {
    aggregateType: &ua::NodeId,
    #[from_inner]
    startTime: ua::DateTime,
    #[skip]
    aggregateConfiguration: &ua::AggregateConfiguration
});

impl MonitoringFilter for AggregateFilter {
    fn to_extension_object(&self) -> ua::ExtensionObject {
        ua::ExtensionObject::new(self)
    }
}
