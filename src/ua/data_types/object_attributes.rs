use crate::{ua, DataType};

crate::data_type!(ObjectAttributes);

impl ObjectAttributes {}

impl Default for ObjectAttributes {
    fn default() -> Self {
        let attrs = unsafe { &open62541_sys::UA_ObjectAttributes_default };

        Self(ObjectAttributes::copy_ua_object_attributes(attrs))
    }
}

impl ObjectAttributes {
    #[must_use]
    pub fn copy_ua_object_attributes(
        attrs: &open62541_sys::UA_ObjectAttributes,
    ) -> open62541_sys::UA_ObjectAttributes {
        open62541_sys::UA_ObjectAttributes {
            specifiedAttributes: attrs.specifiedAttributes,
            displayName: ua::LocalizedText::clone_raw(&attrs.displayName).into_raw(),
            description: ua::LocalizedText::clone_raw(&attrs.description).into_raw(),
            writeMask: attrs.writeMask,
            userWriteMask: attrs.userWriteMask,
            eventNotifier: attrs.eventNotifier,
        }
    }
}
