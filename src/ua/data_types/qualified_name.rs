use crate::{ua, DataType as _};

crate::data_type!(QualifiedName, UA_QualifiedName, UA_TYPES_QUALIFIEDNAME);

impl QualifiedName {
    #[must_use]
    pub const fn namespace_index(&self) -> u16 {
        self.0.namespaceIndex
    }

    #[must_use]
    pub fn name(&self) -> ua::String {
        ua::String::from_ref(&self.0.name)
    }

    #[allow(clippy::inherent_to_string_shadow_display)] // TODO: Fix conflicting definitions.
    #[must_use]
    pub fn to_string(&self) -> String {
        let namespace_index = self.namespace_index();
        if namespace_index == 0 {
            return self.name().to_string().to_string();
        }
        format!("{namespace_index}:{}", self.name().to_string())
    }
}
