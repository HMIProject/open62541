use crate::{MonitoringFilter, ua};

crate::data_type!(DataChangeFilter);
crate::member_accessors!(DataChangeFilter {
    deadbandType: u32,
    deadbandValue: f64,
    #[enum]
    trigger: ua::DataChangeTrigger
});

impl MonitoringFilter for DataChangeFilter {
    fn to_extension_object(&self) -> ua::ExtensionObject {
        ua::ExtensionObject::new(self)
    }
}
