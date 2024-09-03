use crate::ua;

pub enum UserIdentityToken {
    Anonymous(ua::AnonymousIdentityToken),
    UserName(ua::UserNameIdentityToken),
}

impl UserIdentityToken {
    pub(crate) fn to_extension_object(&self) -> ua::ExtensionObject {
        match self {
            UserIdentityToken::Anonymous(anonymous) => ua::ExtensionObject::new(anonymous),
            UserIdentityToken::UserName(user_name) => ua::ExtensionObject::new(user_name),
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
