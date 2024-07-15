use crate::data_type::DataType;

use crate::ua::QualifiedName;

crate::data_type!(RelativePathElement);

impl RelativePathElement {
    #[must_use]
    pub fn new() -> Self {
        Self::clone_raw(unsafe { &(*open62541_sys::UA_RelativePathElement_new()) })
    }

    pub fn init(&mut self) -> &mut Self {
        unsafe { open62541_sys::UA_RelativePathElement_init(self.as_mut_ptr()) }
        self
    }

    pub fn reference_type_id(&mut self, reference_type_id: &crate::ua::NodeId) -> &mut Self {
        // SAFETY: The C code doesn't handle any memory so we cannot give any ownership to it.
        // In the tutorials from open62541 only UA_NODEID_NUMERIC is used, so no string heap
        // allocation, which means everything is on the stack and we can safely use as_raw(),
        // as the instance of this object will only live as long as the reference_type_id.
        self.0.referenceTypeId = unsafe { DataType::to_raw_copy(reference_type_id) };
        self
    }

    pub fn is_inverse(&mut self, is_inverse: bool) -> &mut Self {
        self.0.isInverse = is_inverse;
        self
    }

    pub fn include_subtypes(&mut self, include_subtypes: bool) -> &mut Self {
        self.0.includeSubtypes = include_subtypes;
        self
    }

    pub fn target_name(&mut self, qualified_name: QualifiedName) -> &mut Self {
        // SAFETY: pass ownership of qualified_name to self, so it will be freed when self will be freed,
        // instead of being freed separately and when self is freed.
        self.0.targetName = qualified_name.into_raw();
        self
    }
}

impl Default for RelativePathElement {
    fn default() -> Self {
        Self::new()
    }
}
