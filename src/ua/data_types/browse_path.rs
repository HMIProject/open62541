use crate::data_type::DataType;
use crate::ua;
use crate::ua::NodeId;

crate::data_type!(BrowsePath);

impl BrowsePath {
    pub fn new() -> Self {
        Self::clone_raw(unsafe { &(*open62541_sys::UA_BrowsePath_new()) })
    }

    pub fn init(&mut self) -> &mut Self {
        unsafe { open62541_sys::UA_BrowsePath_init(self.as_mut_ptr()) }
        self
    }

    pub fn starting_node(&mut self, node_id: &NodeId) -> &mut Self {
        self.0.startingNode = unsafe { DataType::to_raw_copy(node_id) };
        self
    }

    pub fn relative_path_element_size(&mut self, element_size: usize) -> &mut Self {
        self.0.relativePath.elementsSize = element_size;
        self
    }

    pub fn relative_path_elements(&mut self, elements: ua::RelativePathElement) -> &mut Self {
        // SAFETY: Pass ownership to self so the RelativePathElement will be freed when self will be freed
        // otherwise the RelativePathElement would have already been freed by the time self would be.
        self.0.relativePath.elements = elements.leak_into_raw();
        self
    }
}
