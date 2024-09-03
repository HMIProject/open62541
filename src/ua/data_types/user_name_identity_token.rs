use crate::{ua, DataType as _};

crate::data_type!(UserNameIdentityToken);

impl UserNameIdentityToken {
    #[must_use]
    pub fn new(user_name: &str, password: &str) -> Self {
        Self::init()
            .with_user_name(user_name)
            .with_password(password)
    }

    /// Sets user name.
    ///
    /// # Panics
    ///
    /// The string must not contain any NUL bytes.
    #[must_use]
    pub fn with_user_name(mut self, user_name: &str) -> Self {
        ua::String::new(user_name)
            .unwrap()
            .move_into_raw(&mut self.0.userName);
        self
    }

    /// Sets password.
    ///
    /// # Panics
    ///
    /// The string must not contain any NUL bytes.
    #[must_use]
    pub fn with_password(mut self, password: &str) -> Self {
        ua::String::new(password)
            .unwrap()
            .move_into_raw(&mut self.0.password);
        self
    }
}
