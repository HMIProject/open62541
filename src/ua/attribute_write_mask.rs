use crate::{ua, DataTypeExt};

/// Wrapper for attribute write mask from [`open62541_sys`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttributeWriteMask(u32);

// See <https://reference.opcfoundation.org/Core/Part3/v105/docs/8.60> for bit values.
impl AttributeWriteMask {
    pub(crate) const fn from_u32(mask: u32) -> Self {
        Self(mask)
    }

    pub(crate) const fn as_u32(&self) -> u32 {
        self.0
    }

    /// Indicates if the AccessLevel attribute is writeable.
    pub const fn access_level(&self) -> bool {
        self.0 & (1 << 0) != 0
    }

    /// Indicates if the ArrayDimensions attribute is writeable.
    pub const fn array_dimensions(&self) -> bool {
        self.0 & (1 << 1) != 0
    }

    /// Indicates if the BrowseName attribute is writeable.
    pub const fn browse_name(&self) -> bool {
        self.0 & (1 << 2) != 0
    }

    /// Indicates if the ContainsNoLoops attribute is writeable.
    pub const fn contains_no_loops(&self) -> bool {
        self.0 & (1 << 3) != 0
    }

    /// Indicates if the DataType attribute is writeable.
    pub const fn data_type(&self) -> bool {
        self.0 & (1 << 4) != 0
    }

    /// Indicates if the Description attribute is writeable.
    pub const fn description(&self) -> bool {
        self.0 & (1 << 5) != 0
    }

    /// Indicates if the DisplayName attribute is writeable.
    pub const fn display_name(&self) -> bool {
        self.0 & (1 << 6) != 0
    }

    /// Indicates if the EventNotifier attribute is writeable.
    pub const fn event_notifier(&self) -> bool {
        self.0 & (1 << 7) != 0
    }

    /// Indicates if the Executable attribute is writeable.
    pub const fn executable(&self) -> bool {
        self.0 & (1 << 8) != 0
    }

    /// Indicates if the Historizing attribute is writeable.
    pub const fn historizing(&self) -> bool {
        self.0 & (1 << 9) != 0
    }

    /// Indicates if the InverseName attribute is writeable.
    pub const fn inverse_name(&self) -> bool {
        self.0 & (1 << 10) != 0
    }

    /// Indicates if the IsAbstract attribute is writeable.
    pub const fn is_abstract(&self) -> bool {
        self.0 & (1 << 11) != 0
    }

    /// Indicates if the MinimumSamplingInterval attribute is writeable.
    pub const fn minimum_sampling_interval(&self) -> bool {
        self.0 & (1 << 12) != 0
    }

    /// Indicates if the NodeClass attribute is writeable.
    pub const fn node_class(&self) -> bool {
        self.0 & (1 << 13) != 0
    }

    /// Indicates if the NodeId attribute is writeable.
    pub const fn node_id(&self) -> bool {
        self.0 & (1 << 14) != 0
    }

    /// Indicates if the Symmetric attribute is writeable.
    pub const fn symmetric(&self) -> bool {
        self.0 & (1 << 15) != 0
    }

    /// Indicates if the UserAccessLevel attribute is writeable.
    pub const fn user_access_level(&self) -> bool {
        self.0 & (1 << 16) != 0
    }

    /// Indicates if the UserExecutable attribute is writeable.
    pub const fn user_executable(&self) -> bool {
        self.0 & (1 << 17) != 0
    }

    /// Indicates if the UserWriteMask attribute is writeable.
    pub const fn user_write_mask(&self) -> bool {
        self.0 & (1 << 18) != 0
    }

    /// Indicates if the ValueRank attribute is writeable.
    pub const fn value_rank(&self) -> bool {
        self.0 & (1 << 19) != 0
    }

    /// Indicates if the WriteMask attribute is writeable.
    pub const fn write_mask(&self) -> bool {
        self.0 & (1 << 20) != 0
    }

    /// Indicates if the Value Attribute is writeable for a VariableType.
    ///
    /// Note: It does not apply for Variables since this is handled by the AccessLevel and
    /// UserAccessLevel attributes for the Variable. For Variables this bit shall be set to 0.
    pub const fn value_for_variable_type(&self) -> bool {
        self.0 & (1 << 21) != 0
    }

    /// Indicates if the DataTypeDefinition attribute is writeable.
    pub const fn data_type_definition(&self) -> bool {
        self.0 & (1 << 22) != 0
    }

    /// Indicates if the RolePermissions attribute is writeable.
    pub const fn role_permissions(&self) -> bool {
        self.0 & (1 << 23) != 0
    }

    /// Indicates if the AccessRestrictions attribute is writeable.
    pub const fn access_restrictions(&self) -> bool {
        self.0 & (1 << 24) != 0
    }

    /// Indicates if the AccessLevelEx attribute is writeable.
    pub const fn access_level_ex(&self) -> bool {
        self.0 & (1 << 25) != 0
    }
}

impl DataTypeExt for AttributeWriteMask {
    type Inner = ua::UInt32;

    fn from_inner(value: Self::Inner) -> Self {
        Self::from_u32(value.value())
    }

    fn into_inner(self) -> Self::Inner {
        Self::Inner::new(self.as_u32())
    }
}
