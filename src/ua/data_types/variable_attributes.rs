use crate::{ua, DataType};

crate::data_type!(VariableAttributes);

impl VariableAttributes {}

impl Default for VariableAttributes {
    fn default() -> Self {
        let attrs = unsafe { &open62541_sys::UA_VariableAttributes_default };

        Self(VariableAttributes::copy_ua_variable_attributes(attrs))
    }
}

impl VariableAttributes {
    #[must_use]
    pub fn copy_ua_variable_attributes(
        attrs: &open62541_sys::UA_VariableAttributes,
    ) -> open62541_sys::UA_VariableAttributes {
        open62541_sys::UA_VariableAttributes {
            specifiedAttributes: attrs.specifiedAttributes,
            displayName: ua::LocalizedText::clone_raw(&attrs.displayName).into_raw(),
            description: ua::LocalizedText::clone_raw(&attrs.description).into_raw(),
            writeMask: attrs.writeMask,
            userWriteMask: attrs.userWriteMask,
            value: ua::Variant::clone_raw(&attrs.value).into_raw(),
            dataType: ua::NodeId::clone_raw(&attrs.dataType).into_raw(),
            valueRank: attrs.valueRank,
            arrayDimensionsSize: attrs.arrayDimensionsSize,
            arrayDimensions: attrs.arrayDimensions,
            accessLevel: attrs.accessLevel,
            userAccessLevel: attrs.userAccessLevel,
            minimumSamplingInterval: attrs.minimumSamplingInterval,
            historizing: attrs.historizing,
        }
    }
}
