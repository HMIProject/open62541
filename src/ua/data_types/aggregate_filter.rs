use crate::{MonitoringFilter, ua};

crate::data_type!(AggregateFilter);

impl MonitoringFilter for AggregateFilter {
    fn to_extension_object(&self) -> ua::ExtensionObject {
        ua::ExtensionObject::new(self)
    }
}
