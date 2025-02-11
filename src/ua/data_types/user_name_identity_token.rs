use crate::{ua, DataType as _};

crate::data_type!(UserNameIdentityToken);

impl UserNameIdentityToken {
    /// Creates token with user name and password.
    ///
    /// This is a shortcut for calling [`Self::with_user_name()`] and [`Self::with_password()`].
    ///
    /// # Panics
    ///
    /// The user name must not contain any NUL bytes.
    #[must_use]
    pub fn new(user_name: &str, password: &str) -> Self {
        Self::init()
            .with_user_name(ua::String::new(user_name).unwrap())
            .with_password(ua::ByteString::new(password.as_bytes()))
    }

    /// Sets policy ID.
    #[must_use]
    pub fn with_policy_id(mut self, policy_id: ua::String) -> Self {
        policy_id.move_into_raw(&mut self.0.policyId);
        self
    }

    /// Sets user name.
    #[must_use]
    pub fn with_user_name(mut self, user_name: ua::String) -> Self {
        user_name.move_into_raw(&mut self.0.userName);
        self
    }

    /// Sets password.
    #[must_use]
    pub fn with_password(mut self, password: ua::ByteString) -> Self {
        password.move_into_raw(&mut self.0.password);
        self
    }

    /// Sets encryption algorithm.
    #[must_use]
    pub fn with_encryption_algorithm(mut self, encryption_algorithm: ua::String) -> Self {
        encryption_algorithm.move_into_raw(&mut self.0.encryptionAlgorithm);
        self
    }
}
