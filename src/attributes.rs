use crate::{ua, Attribute};

macro_rules! attribute_impl {
    ($( ($name:ident, $value:ident) ),* $(,)?) => {
        $(
            #[derive(Debug, Clone, Copy)]
            pub struct $name;

            impl $crate::Attribute for $name {
                type Value = $crate::ua::$value;

                fn id(&self) -> $crate::ua::AttributeId {
                    paste::paste! {
                        $crate::ua::AttributeId::[<$name:upper>]
                    }
                }
            }

            impl $crate::ua::AttributeId {
                paste::paste! {
                    /// Implementation of [`crate::Attribute`] for
                    #[doc = concat!("[`", stringify!([<$name:upper>]), "`](Self::", stringify!([<$name:upper>]), ").")]
                    pub const [<$name:upper _T>]: $crate::attributes::$name =
                        $crate::attributes::$name;
                }
            }
        )*
    };
}

attribute_impl!(
    (NodeId, NodeId),
    (NodeClass, NodeClass),
    (BrowseName, QualifiedName),
    (DisplayName, LocalizedText),
    (Description, LocalizedText),
    (WriteMask, UInt32),
    (IsAbstract, Boolean),
    (Symmetric, Boolean),
    (InverseName, LocalizedText),
    (ContainsNoLoops, Boolean),
    (EventNotifier, Byte),
    (Value, Variant),
    (DataType, NodeId),
    (ValueRank, UInt32),
    (ArrayDimensions, Variant),
    (AccessLevel, Byte),
    (AccessLevelEx, UInt32),
    (MinimumSamplingInterval, Double),
    (Historizing, Boolean),
    (Executable, Boolean),
);

impl Attribute for &ua::AttributeId {
    type Value = ua::Variant;

    fn id(&self) -> ua::AttributeId {
        (*self).clone()
    }
}
