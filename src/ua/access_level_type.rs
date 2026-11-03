crate::data_type_baseline!(
    #[expect(
        clippy::derived_hash_with_manual_eq,
        reason = "open62541 numeric equality impl satisfies `Hash`s identity invariant"
    )]
    #[derive(Clone, Copy, Hash)]
    AccessLevelType
);

// See <https://reference.opcfoundation.org/Core/Part3/v104/docs/8.57> for bit values.
crate::enum_bitmask!(
    enum AccessLevelType : u8 {
        /// Indicates if the current value is readable.
        ///
        /// It also indicates if the current value of the variable is available.
        CurrentRead,
        /// Indicates if the current value is writable.
        ///
        /// It also indicates if the current value of the variable is available.
        CurrentWrite,
        /// Indicates if the history of the value is readable.
        ///
        /// It also indicates if the history of the variable is available via the OPC UA Server.
        HistoryRead,
        /// Indicates if the history of the value is writable.
        ///
        /// It also indicates if the history of the variable is available via the OPC UA Server.
        HistoryWrite,
        /// This flag is set for properties that define semantic aspects of the parent node of the
        /// property and where the property value, and thus the semantic, may change during operation.
        SemanticChange,
        /// Indicates if the current status code of the value is writable.
        StatusWrite,
        /// Indicates if the current source timestamp is writable.
        TimestampWrite,
    }
);

#[deprecated = "Use AccessLevelType instead."]
pub type AccessLevel = AccessLevelType;
