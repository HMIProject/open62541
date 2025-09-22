use crate::{Attribute, ua};

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

// Attribute types taken from <https://reference.opcfoundation.org/Core/Part3/v105/docs/5>.
//
// Note: Array values are not supported yet in their typed form: previously, any such attempt would
// fail, because converting to `DataValue` expects scalar values.
//
// To give us some time to think about the best, typed representation of such non-scalar values, we
// remove their `impl` for now. Access is still possible with the non-typed attribute methods.
attribute_impl!(
    (NodeId, NodeId),
    (NodeClass, NodeClass),
    (BrowseName, QualifiedName),
    (DisplayName, LocalizedText),
    (Description, LocalizedText),
    (WriteMask, AttributeWriteMask),
    (UserWriteMask, AttributeWriteMask),
    (IsAbstract, Boolean),
    (Symmetric, Boolean),
    (InverseName, LocalizedText),
    (ContainsNoLoops, Boolean),
    (EventNotifier, EventNotifierType),
    (Value, Variant),
    (DataType, NodeId),
    (ValueRank, Int32),
    // (ArrayDimensions, UInt32[]),
    (AccessLevel, AccessLevelType),
    (UserAccessLevel, AccessLevelType),
    (MinimumSamplingInterval, Duration),
    (Historizing, Boolean),
    (Executable, Boolean),
    (UserExecutable, Boolean),
    (DataTypeDefinition, DataTypeDefinition),
    // (RolePermissions, RolePermissionType[]),`
    // (UserRolePermissions, RolePermissionType[]),`
    (AccessRestrictions, AccessRestrictionType),
    (AccessLevelEx, AccessLevelExType),
);

impl Attribute for &ua::AttributeId {
    type Value = ua::Variant;

    fn id(&self) -> ua::AttributeId {
        (*self).clone()
    }
}
