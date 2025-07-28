use std::{ffi::CString, fmt, hash};

use open62541_sys::{UA_QualifiedName_hash, UA_QUALIFIEDNAME_ALLOC};

use crate::{ua, DataType as _};

crate::data_type!(QualifiedName);

impl QualifiedName {
    /// Creates qualified name.
    ///
    /// # Panics
    ///
    /// The string must not contain any NUL bytes.
    #[must_use]
    pub fn new(namespace_index: u16, name: &str) -> Self {
        let name = CString::new(name).expect("string does not contain NUL bytes");

        let inner = unsafe { UA_QUALIFIEDNAME_ALLOC(namespace_index, name.as_ptr()) };
        if !name.is_empty() && (inner.name.data.is_null() || inner.name.length == 0) {
            debug_assert!(inner.name.data.is_null(), "unexpected string data");
            panic!("string should have been allocated");
        }

        Self(inner)
    }

    /// Creates qualified name in namespace 0.
    ///
    /// Namespace 0 is always the UA namespace `http://opcfoundation.org/UA/` itself and is used for
    /// fixed definitions as laid out in the OPC UA specification.
    #[must_use]
    pub fn ns0(name: &str) -> Self {
        Self::new(0, name)
    }

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

impl hash::Hash for QualifiedName {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        // We are using UA_QualifiedName_hash() instead of a custom u64 hash function
        // for consistency with UA_QualifiedName_equal().
        let hash = unsafe { UA_QualifiedName_hash(self.as_ptr()) };

        state.write_u32(hash);
    }
}

impl fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Replace custom implementation with UA_QualifiedName_print() when available.
        let namespace_index = self.namespace_index();
        if namespace_index == 0 {
            return self.name().fmt(f);
        }
        write!(f, "{namespace_index}:{}", self.name())
    }
}

// TODO: Implement std::str::FromStr for QualifiedName using UA_QualifiedName_parse() when available.

#[cfg(test)]
mod tests {
    use crate::ua;

    #[test]
    fn value_representation() {
        let name = ua::QualifiedName::new(123, "lorem");

        // We get the original values back.
        //
        assert_eq!(name.namespace_index(), 123);
        assert_eq!(name.name().as_str(), Some("lorem"));
    }
}
