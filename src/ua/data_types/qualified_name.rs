use crate::ua;

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

    pub fn to_string(&self) -> String {
        let namespace_index = self.namespace_index();
        if namespace_index == 0 {
            return self.name().to_string().to_string();
        }
        format!("{namespace_index}:{}", self.name().to_string())
    }
}
