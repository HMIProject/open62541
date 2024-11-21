use std::ptr;

use open62541_sys::{UA_AccessControl_default, UA_ServerConfig, UA_UsernamePasswordLogin};

use crate::{ua, DataType, Error, Result};

/// Server access control.
pub trait AccessControl {
    /// Consumes instance and applies it to config.
    ///
    /// # Errors
    ///
    /// This fails when the access control cannot be applied.
    fn apply(self, config: &mut UA_ServerConfig) -> Result<()>;
}

/// Default server access control.
///
/// This uses `UA_AccessControl_default()` which comes with the following warning:
///
/// > Example access control management. Anonymous and username/password login. The access rights
/// > are maximally permissive.
/// >
/// > FOR PRODUCTION USE, THIS EXAMPLE PLUGIN SHOULD BE REPLACED WITH LESS PERMISSIVE ACCESS
/// > CONTROL.
/// >
/// > For `TransferSubscriptions`, we check whether the transfer happens between Sessions for the
/// > same user.
#[allow(missing_debug_implementations)] // Do not leak credentials.
pub struct DefaultAccessControl<'a> {
    allow_anonymous: bool,
    username_password_login: &'a [(&'a ua::String, &'a ua::String)],
}

impl<'a> DefaultAccessControl<'a> {
    /// Creates default access control.
    #[must_use]
    pub const fn new(
        allow_anonymous: bool,
        username_password_login: &'a [(&'a ua::String, &'a ua::String)],
    ) -> Self {
        Self {
            allow_anonymous,
            username_password_login,
        }
    }
}

impl<'a> AccessControl for DefaultAccessControl<'a> {
    fn apply(self, config: &mut UA_ServerConfig) -> Result<()> {
        let Self {
            allow_anonymous,
            username_password_login,
        } = self;

        let username_password_login = username_password_login
            .iter()
            // SAFETY: `UA_AccessControl_default()` does not take ownership of strings. It uses them
            // only to make internal copies.
            .map(|(username, password)| unsafe {
                UA_UsernamePasswordLogin {
                    username: DataType::to_raw_copy(*username),
                    password: DataType::to_raw_copy(*password),
                }
            })
            .collect::<Vec<_>>();

        let status_code = ua::StatusCode::new(unsafe {
            UA_AccessControl_default(
                config,
                allow_anonymous,
                ptr::null(),
                username_password_login.len(),
                username_password_login.as_ptr(),
            )
        });
        Error::verify_good(&status_code)
    }
}
