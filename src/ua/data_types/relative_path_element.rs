use crate::{ua, DataType as _};

crate::data_type!(RelativePathElement);

impl RelativePathElement {
    #[must_use]
    pub fn with_reference_type_id(mut self, reference_type_id: &ua::NodeId) -> Self {
        reference_type_id.clone_into_raw(&mut self.0.referenceTypeId);
        self
    }

    #[must_use]
    pub fn with_is_inverse(mut self, is_inverse: bool) -> Self {
        self.0.isInverse = is_inverse;
        self
    }

    #[must_use]
    pub fn with_include_subtypes(mut self, include_subtypes: bool) -> Self {
        self.0.includeSubtypes = include_subtypes;
        self
    }

    #[must_use]
    pub fn with_target_name(mut self, target_name: &ua::QualifiedName) -> Self {
        target_name.clone_into_raw(&mut self.0.targetName);
        self
    }
}
