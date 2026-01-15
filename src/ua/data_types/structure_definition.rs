use std::slice;

use crate::{DataType, ua};

crate::data_type!(StructureDefinition);

impl StructureDefinition {
    #[must_use]
    pub fn default_encoding_id(&self) -> &ua::NodeId {
        ua::NodeId::raw_ref(&self.0.defaultEncodingId)
    }

    #[must_use]
    pub fn base_data_type(&self) -> &ua::NodeId {
        ua::NodeId::raw_ref(&self.0.baseDataType)
    }

    #[must_use]
    pub fn fields(&self) -> Option<ua::Array<ua::StructureField>> {
        // TODO: Adjust signature to return non-owned value instead.
        ua::Array::from_raw_parts(self.0.fieldsSize, self.0.fields)
    }

    pub fn drop_arrays(&mut self) {
        if self.0.fieldsSize == 0 || self.0.fields.is_null() {
            return;
        }

        let fields = unsafe { slice::from_raw_parts(self.0.fields, self.0.fieldsSize) };

        if fields.last().unwrap().arrayDimensionsSize != 0 {
            self.0.fieldsSize -= 1;
        }
    }

    pub fn replace_data_type(&mut self, from: &ua::NodeId, to: &ua::NodeId) {
        let Some(fields) = (unsafe {
            ua::Array::<ua::StructureField>::slice_from_raw_parts_mut(
                self.0.fieldsSize,
                self.0.fields,
            )
        }) else {
            return;
        };

        for field in fields {
            if field.data_type() == from {
                field.set_data_type(to.clone());
            }
        }
    }

    #[must_use]
    pub fn into_description(
        self,
        data_type_id: ua::NodeId,
        name: ua::QualifiedName,
    ) -> ua::StructureDescription {
        ua::StructureDescription::new(data_type_id, name, self)
    }
}
