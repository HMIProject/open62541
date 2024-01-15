use std::fmt;

use crate::{ua, DataType as _};

crate::data_type!(QualifiedName);

impl QualifiedName {
    #[must_use]
    pub const fn namespace_index(&self) -> u16 {
        self.0.namespaceIndex
    }

    #[must_use]
    pub fn name(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.name)
    }
}

impl fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let namespace_index = self.namespace_index();
        if namespace_index == 0 {
            return write!(f, "{}", self.name());
        }
        write!(f, "{namespace_index}:{}", self.name())
    }
}
