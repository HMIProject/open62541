use crate::{MonitoringFilter, ua};

crate::data_type!(DataChangeFilter);

impl MonitoringFilter for DataChangeFilter {
    fn to_extension_object(&self) -> ua::ExtensionObject {
        ua::ExtensionObject::new(self)
    }
}
