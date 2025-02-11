use crate::ua;

#[derive(Debug)]
pub enum UserIdentityToken {
    Anonymous(ua::AnonymousIdentityToken),
    UserName(ua::UserNameIdentityToken),
    X509(ua::X509IdentityToken),
}

impl UserIdentityToken {
    pub(crate) fn to_extension_object(&self) -> ua::ExtensionObject {
        match self {
            Self::Anonymous(anonymous) => ua::ExtensionObject::new(anonymous),
            Self::UserName(user_name) => ua::ExtensionObject::new(user_name),
            Self::X509(x509) => ua::ExtensionObject::new(x509),
        }
    }
}

impl From<ua::AnonymousIdentityToken> for UserIdentityToken {
    fn from(value: ua::AnonymousIdentityToken) -> Self {
        Self::Anonymous(value)
    }
}

impl From<ua::UserNameIdentityToken> for UserIdentityToken {
    fn from(value: ua::UserNameIdentityToken) -> Self {
        Self::UserName(value)
    }
}

impl From<ua::X509IdentityToken> for UserIdentityToken {
    fn from(value: ua::X509IdentityToken) -> Self {
        Self::X509(value)
    }
}
