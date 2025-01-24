use crate::{ua, MonitoringFilter};

crate::data_type!(DataChangeFilter);

impl MonitoringFilter for DataChangeFilter {
    fn to_extension_object(&self) -> ua::ExtensionObject {
        ua::ExtensionObject::new(self)
    }
}
