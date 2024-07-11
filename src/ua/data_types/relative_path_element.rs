use crate::data_type::DataType;

crate::data_type!(RelativePathElement);

impl RelativePathElement {
    pub fn new() -> Self {
        Self::clone_raw(unsafe { &(*open62541_sys::UA_RelativePathElement_new()) })
    }

    pub fn init(&mut self) -> &mut Self {
        unsafe { open62541_sys::UA_RelativePathElement_init(self.as_mut_ptr()) }
        self
    }

    pub fn referenceTypeId(&mut self, reference_type_id: crate::ua::NodeId) -> &mut Self {
        // SAFETY: The C code doesn't handle any memory so we cannot give any ownership to it.
        // In the tutorials from open62541 only UA_NODEID_NUMERIC is used, so no string heap
        // allocation, which means everything is on the stack and we can safely use as_raw(),
        // as the instance of this object will only live as long as the reference_type_id.
        self.0.referenceTypeId = unsafe { reference_type_id.as_raw() };
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

    pub fn target_name(&mut self, namespace_index: u16, target_name: &str) -> &mut Self {
        // SAFETY: the new qualified name here will hopefully not be destroyed after this
        // function returns otherwise that will be a problem.
        self.0.targetName =
            unsafe { crate::ua::QualifiedName::new(namespace_index, target_name).as_raw() };
        self
    }
}
