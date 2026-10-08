use crate::{MonitoringFilter, ua};
use open62541_sys::UA_DataChangeTrigger;

crate::data_type!(DataChangeFilter);
crate::member_accessors!(DataChangeFilter {
    deadbandType: u32,
    deadbandValue: f64,
    #[enum(UA_DataChangeTrigger)]
    trigger: ua::DataChangeTrigger
});

impl MonitoringFilter for DataChangeFilter {
    fn to_extension_object(&self) -> ua::ExtensionObject {
        ua::ExtensionObject::new(self)
    }
}
