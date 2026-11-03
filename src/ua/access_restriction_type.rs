crate::data_type_baseline!(
    #[expect(
        clippy::derived_hash_with_manual_eq,
        reason = "open62541 numeric equality impl satisfies `Hash`s identity invariant"
    )]
    #[derive(Clone, Copy, Hash)]
    AccessRestrictionType
);

// See <https://reference.opcfoundation.org/Core/Part3/v105/docs/8.56> for bit values.
crate::enum_bitmask!(
    enum AccessRestrictionType : u16 {
        /// If this bit is set, the access restrictions `SigningRequired` and `EncryptionRequired` are
        /// also applied to the browse permission.
        ApplyRestrictionsToBrowse,
        /// The client can only access the node when using a secure channel which encrypts all messages.
        ///
        /// This does not apply to the browse permission if the `ApplyRestrictionsToBrowse` is not set.
        EncryptionRequired,
        /// The client cannot access the node when using `SessionlessInvoke` service invocation.
        SessionRequired,
        /// The client can only access the node when using a secure channel which digitally signs all
        /// messages.
        ///
        /// This does not apply to the browse permission if the `ApplyRestrictionsToBrowse` is not set.
        SigningRequired,
    }
);
