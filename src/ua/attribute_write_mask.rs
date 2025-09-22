use open62541_sys::{
    UA_ATTRIBUTEWRITEMASK_ACCESSLEVEL, UA_ATTRIBUTEWRITEMASK_ACCESSLEVELEX,
    UA_ATTRIBUTEWRITEMASK_ACCESSRESTRICTIONS, UA_ATTRIBUTEWRITEMASK_ARRAYDIMENSIONS,
    UA_ATTRIBUTEWRITEMASK_BROWSENAME, UA_ATTRIBUTEWRITEMASK_CONTAINSNOLOOPS,
    UA_ATTRIBUTEWRITEMASK_DATATYPE, UA_ATTRIBUTEWRITEMASK_DATATYPEDEFINITION,
    UA_ATTRIBUTEWRITEMASK_DESCRIPTION, UA_ATTRIBUTEWRITEMASK_DISPLAYNAME,
    UA_ATTRIBUTEWRITEMASK_EVENTNOTIFIER, UA_ATTRIBUTEWRITEMASK_EXECUTABLE,
    UA_ATTRIBUTEWRITEMASK_HISTORIZING, UA_ATTRIBUTEWRITEMASK_INVERSENAME,
    UA_ATTRIBUTEWRITEMASK_ISABSTRACT, UA_ATTRIBUTEWRITEMASK_MINIMUMSAMPLINGINTERVAL,
    UA_ATTRIBUTEWRITEMASK_NODECLASS, UA_ATTRIBUTEWRITEMASK_NODEID,
    UA_ATTRIBUTEWRITEMASK_ROLEPERMISSIONS, UA_ATTRIBUTEWRITEMASK_SYMMETRIC,
    UA_ATTRIBUTEWRITEMASK_USERACCESSLEVEL, UA_ATTRIBUTEWRITEMASK_USEREXECUTABLE,
    UA_ATTRIBUTEWRITEMASK_USERWRITEMASK, UA_ATTRIBUTEWRITEMASK_VALUEFORVARIABLETYPE,
    UA_ATTRIBUTEWRITEMASK_VALUERANK, UA_ATTRIBUTEWRITEMASK_WRITEMASK, UA_AttributeWriteMask,
};

use crate::{DataTypeExt, ua};

/// Wrapper for [`UA_AttributeWriteMask`] from [`open62541_sys`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttributeWriteMask(UA_AttributeWriteMask);

// See <https://reference.opcfoundation.org/Core/Part3/v105/docs/8.60> for bit values.
impl AttributeWriteMask {
    pub(crate) const fn from_u32(mask: u32) -> Self {
        Self(mask)
    }

    pub(crate) const fn as_u32(&self) -> u32 {
        self.0
    }

    /// Indicates if the `AccessLevel` attribute is writeable.
    #[must_use]
    pub const fn access_level(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_ACCESSLEVEL != 0
    }

    /// Indicates if the `ArrayDimensions` attribute is writeable.
    #[must_use]
    pub const fn array_dimensions(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_ARRAYDIMENSIONS != 0
    }

    /// Indicates if the `BrowseName` attribute is writeable.
    #[must_use]
    pub const fn browse_name(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_BROWSENAME != 0
    }

    /// Indicates if the `ContainsNoLoops` attribute is writeable.
    #[must_use]
    pub const fn contains_no_loops(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_CONTAINSNOLOOPS != 0
    }

    /// Indicates if the `DataType` attribute is writeable.
    #[must_use]
    pub const fn data_type(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_DATATYPE != 0
    }

    /// Indicates if the `Description` attribute is writeable.
    #[must_use]
    pub const fn description(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_DESCRIPTION != 0
    }

    /// Indicates if the `DisplayName` attribute is writeable.
    #[must_use]
    pub const fn display_name(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_DISPLAYNAME != 0
    }

    /// Indicates if the `EventNotifier` attribute is writeable.
    #[must_use]
    pub const fn event_notifier(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_EVENTNOTIFIER != 0
    }

    /// Indicates if the `Executable` attribute is writeable.
    #[must_use]
    pub const fn executable(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_EXECUTABLE != 0
    }

    /// Indicates if the `Historizing` attribute is writeable.
    #[must_use]
    pub const fn historizing(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_HISTORIZING != 0
    }

    /// Indicates if the `InverseName` attribute is writeable.
    #[must_use]
    pub const fn inverse_name(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_INVERSENAME != 0
    }

    /// Indicates if the `IsAbstract` attribute is writeable.
    #[must_use]
    pub const fn is_abstract(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_ISABSTRACT != 0
    }

    /// Indicates if the `MinimumSamplingInterval` attribute is writeable.
    #[must_use]
    pub const fn minimum_sampling_interval(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_MINIMUMSAMPLINGINTERVAL != 0
    }

    /// Indicates if the `NodeClass` attribute is writeable.
    #[must_use]
    pub const fn node_class(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_NODECLASS != 0
    }

    /// Indicates if the `NodeId` attribute is writeable.
    #[must_use]
    pub const fn node_id(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_NODEID != 0
    }

    /// Indicates if the `Symmetric` attribute is writeable.
    #[must_use]
    pub const fn symmetric(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_SYMMETRIC != 0
    }

    /// Indicates if the `UserAccessLevel` attribute is writeable.
    #[must_use]
    pub const fn user_access_level(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_USERACCESSLEVEL != 0
    }

    /// Indicates if the `UserExecutable` attribute is writeable.
    #[must_use]
    pub const fn user_executable(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_USEREXECUTABLE != 0
    }

    /// Indicates if the `UserWriteMask` attribute is writeable.
    #[must_use]
    pub const fn user_write_mask(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_USERWRITEMASK != 0
    }

    /// Indicates if the `ValueRank` attribute is writeable.
    #[must_use]
    pub const fn value_rank(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_VALUERANK != 0
    }

    /// Indicates if the `WriteMask` attribute is writeable.
    #[must_use]
    pub const fn write_mask(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_WRITEMASK != 0
    }

    /// Indicates if the `Value` Attribute is writeable for a `VariableType`.
    ///
    /// Note: It does not apply for variables since this is handled by the `AccessLevel` and
    /// `UserAccessLevel` attributes for the Variable. For variables this bit shall be set to 0.
    #[must_use]
    pub const fn value_for_variable_type(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_VALUEFORVARIABLETYPE != 0
    }

    /// Indicates if the `DataTypeDefinition` attribute is writeable.
    #[must_use]
    pub const fn data_type_definition(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_DATATYPEDEFINITION != 0
    }

    /// Indicates if the `RolePermissions` attribute is writeable.
    #[must_use]
    pub const fn role_permissions(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_ROLEPERMISSIONS != 0
    }

    /// Indicates if the `AccessRestrictions` attribute is writeable.
    #[must_use]
    pub const fn access_restrictions(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_ACCESSRESTRICTIONS != 0
    }

    /// Indicates if the `AccessLevelEx` attribute is writeable.
    #[must_use]
    pub const fn access_level_ex(&self) -> bool {
        self.0 & UA_ATTRIBUTEWRITEMASK_ACCESSLEVELEX != 0
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
