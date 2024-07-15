mod variable_attributes;

use open62541_sys::UA_NodeAttributes;

use crate::{ua, DataType as _};

crate::data_type!(NodeAttributes);

macro_rules! derived {
    ($( $name:ident ),* $(,)?) => {
        $(
            $crate::data_type!($name);

            impl $name {
                #[allow(dead_code)]
                pub(crate) fn as_node_attributes(&self) -> &ua::NodeAttributes {
                    // SAFETY: This transmutes from `Self` to `UA_NodeAttributes`, a strict subset of
                    // `UA_(...)Attributes` with the same memory layout.
                    let node_attributes = unsafe { self.as_ptr().cast::<UA_NodeAttributes>() };
                    // SAFETY: Transmutation is allowed and pointer is valid (non-zero).
                    let node_attributes = unsafe { node_attributes.as_ref().unwrap_unchecked() };
                    ua::NodeAttributes::raw_ref(node_attributes)
                }
            }

            impl Default for $name {
                fn default() -> Self {
                    paste::paste! {
                        Self::clone_raw(unsafe { &open62541_sys::[<UA_ $name _default>] })
                    }
                }
            }
        )*
    };
}

// This adds basic declarations and shared functionality such as upcasting to `ua::NodeAttributes`.
// See sub-modules for type-specific implementations, e.g. `variable_attributes`.
derived!(
    ObjectAttributes,
    VariableAttributes,
    MethodAttributes,
    ObjectTypeAttributes,
    VariableTypeAttributes,
    ReferenceTypeAttributes,
    DataTypeAttributes,
    ViewAttributes,
    // GenericAttributes, // Omitted for now because the `Default` impl above cannot be used here.
);
