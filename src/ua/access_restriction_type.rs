use open62541_sys::{
    UA_ACCESSRESTRICTIONTYPE_APPLYRESTRICTIONSTOBROWSE,
    UA_ACCESSRESTRICTIONTYPE_ENCRYPTIONREQUIRED, UA_ACCESSRESTRICTIONTYPE_SESSIONREQUIRED,
    UA_ACCESSRESTRICTIONTYPE_SIGNINGREQUIRED, UA_AccessRestrictionType,
};

use crate::{DataTypeExt, ua};

/// Wrapper for [`UA_AccessRestrictionType`] from [`open62541_sys`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccessRestrictionType(UA_AccessRestrictionType);

// See <https://reference.opcfoundation.org/Core/Part3/v105/docs/8.56> for bit values.
impl AccessRestrictionType {
    pub(crate) const fn from_u16(mask: u16) -> Self {
        Self(mask)
    }

    pub(crate) const fn as_u16(&self) -> u16 {
        self.0
    }

    /// The client can only access the node when using a secure channel which digitally signs all
    /// messages.
    ///
    /// This does not apply to the browse permission if the `ApplyRestrictionsToBrowse` is not set.
    #[must_use]
    pub fn signing_required(&self) -> bool {
        u32::from(self.as_u16()) & UA_ACCESSRESTRICTIONTYPE_SIGNINGREQUIRED != 0
    }

    /// The client can only access the node when using a secure channel which encrypts all messages.
    ///
    /// This does not apply to the browse permission if the `ApplyRestrictionsToBrowse` is not set.
    #[must_use]
    pub fn encryption_required(&self) -> bool {
        u32::from(self.as_u16()) & UA_ACCESSRESTRICTIONTYPE_ENCRYPTIONREQUIRED != 0
    }

    /// The client cannot access the node when using `SessionlessInvoke` service invocation.
    #[must_use]
    pub fn session_required(&self) -> bool {
        u32::from(self.as_u16()) & UA_ACCESSRESTRICTIONTYPE_SESSIONREQUIRED != 0
    }

    /// If this bit is set, the access restrictions `SigningRequired` and `EncryptionRequired` are
    /// also applied to the browse permission.
    #[must_use]
    pub fn apply_restrictions_to_browse(&self) -> bool {
        u32::from(self.as_u16()) & UA_ACCESSRESTRICTIONTYPE_APPLYRESTRICTIONSTOBROWSE != 0
    }
}

impl DataTypeExt for AccessRestrictionType {
    type Inner = ua::UInt16;

    fn from_inner(value: Self::Inner) -> Self {
        Self::from_u16(value.value())
    }

    fn into_inner(self) -> Self::Inner {
        Self::Inner::new(self.as_u16())
    }
}
