mod variable_attributes;

use open62541_sys::UA_NodeAttributes;

use crate::{ua, DataType as _};

crate::data_type!(NodeAttributes);

macro_rules! derived {
    ($( $name:ident ),* $(,)?) => {
        $(
            paste::paste! {
                $crate::data_type!([<$name Attributes>]);
            }

            impl $crate::Attributes for paste::paste!{[<$name Attributes>]} {
                fn node_class(&self) -> ua::NodeClass {
                    paste::paste! {
                        ua::NodeClass::[<$name:upper>]
                    }
                }

                fn with_display_name(mut self, display_name: &ua::LocalizedText) -> Self {
                    display_name.clone_into_raw(&mut self.0.displayName);
                    self.0.specifiedAttributes |= ua::SpecifiedAttributes::DISPLAYNAME;
                    self
                }

                #[allow(dead_code)]
                fn as_node_attributes(&self) -> &ua::NodeAttributes {
                    // SAFETY: This transmutes from `Self` to `UA_NodeAttributes`, a strict subset of
                    // `UA_(...)Attributes` with the same memory layout.
                    let node_attributes = unsafe { self.as_ptr().cast::<UA_NodeAttributes>() };
                    // SAFETY: Transmutation is allowed and pointer is valid (non-zero).
                    let node_attributes = unsafe { node_attributes.as_ref().unwrap_unchecked() };
                    ua::NodeAttributes::raw_ref(node_attributes)
                }
            }

            impl Default for paste::paste!{[<$name Attributes>]} {
                fn default() -> Self {
                    paste::paste! {
                        Self::clone_raw(unsafe { &open62541_sys::[<UA_ $name Attributes_default>] })
                    }
                }
            }
        )*
    };
}

// This adds basic declarations and shared functionality such as upcasting to `ua::NodeAttributes`.
// See sub-modules for type-specific implementations, e.g. `variable_attributes`.
derived!(
    Object,
    Variable,
    Method,
    ObjectType,
    VariableType,
    ReferenceType,
    DataType,
    View,
    // Generic, // Omitted for now because the `Default` impl above cannot be used here.
);
