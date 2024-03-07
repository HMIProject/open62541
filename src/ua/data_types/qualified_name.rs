use std::fmt;

use crate::{ua, DataType as _};

crate::data_type!(QualifiedName);

impl QualifiedName {
    /// Gets namespace index.
    #[must_use]
    pub const fn namespace_index(&self) -> u16 {
        self.0.namespaceIndex
    }

    /// Gets name.
    #[must_use]
    pub fn name(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.name)
    }

    /// Gets name in namespace 0.
    ///
    /// Namespace 0 is always the UA namespace `http://opcfoundation.org/UA/` itself and is used for
    /// fixed definitions as laid out in the OPC UA specification.
    #[must_use]
    pub fn as_ns0(&self) -> Option<&ua::String> {
        (self.namespace_index() == 0).then(|| self.name())
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
