crate::data_type_baseline! {
    #[expect(
        clippy::derived_hash_with_manual_eq,
        reason = "open62541 numeric equality impl satisfies `Hash`s identity invariant"
    )]
    #[derive(Clone, Copy, Hash)]
    AttributeWriteMask
}

// See <https://reference.opcfoundation.org/Core/Part3/v105/docs/8.60> for bit values.
crate::enum_bitmask!(
    enum AttributeWriteMask : u32 {
        /// Indicates if the `AccessLevel` attribute is writeable.
        AccessLevel,
        /// Indicates if the `AccessLevelEx` attribute is writeable.
        AccessLevelEx,
        /// Indicates if the `AccessRestrictions` attribute is writeable.
        AccessRestrictions,
        /// Indicates if the `ArrayDimensions` attribute is writeable.
        ArrayDimensions,
        /// Indicates if the `BrowseName` attribute is writeable.
        BrowseName,
        /// Indicates if the `ContainsNoLoops` attribute is writeable.
        ContainsNoLoops,
        /// Indicates if the `DataType` attribute is writeable.
        DataType,
        /// Indicates if the `DataTypeDefinition` attribute is writeable.
        DataTypeDefinition,
        /// Indicates if the `Description` attribute is writeable.
        Description,
        /// Indicates if the `DisplayName` attribute is writeable.
        DisplayName,
        /// Indicates if the `EventNotifier` attribute is writeable.
        EventNotifier,
        /// Indicates if the `Executable` attribute is writeable.
        Executable,
        /// Indicates if the `Historizing` attribute is writeable.
        Historizing,
        /// Indicates if the `InverseName` attribute is writeable.
        InverseName,
        /// Indicates if the `IsAbstract` attribute is writeable.
        IsAbstract,
        /// Indicates if the `MinimumSamplingInterval` attribute is writeable.
        MinimumSamplingInterval,
        /// Indicates if the `NodeClass` attribute is writeable.
        NodeClass,
        /// Indicates if the `NodeId` attribute is writeable.
        NodeId,
        /// Indicates if the `RolePermissions` attribute is writeable.
        RolePermissions,
        /// Indicates if the `Symmetric` attribute is writeable.
        Symmetric,
        /// Indicates if the `UserAccessLevel` attribute is writeable.
        UserAccessLevel,
        /// Indicates if the `UserExecutable` attribute is writeable.
        UserExecutable,
        /// Indicates if the `UserWriteMask` attribute is writeable.
        UserWriteMask,
        /// Indicates if the `ValueForVariableType` attribute is writeable.
        ///
        /// Note: It does not apply for variables since this is handled by the `AccessLevel` and
        /// `UserAccessLevel` attributes for the Variable. For variables this bit shall be set to 0.
       ValueForVariableType,
        /// Indicates if the `ValueRank` attribute is writeable.
        ValueRank,
        /// Indicates if the `WriteMask` attribute is writeable.
        WriteMask,
    }
);
