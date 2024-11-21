use std::{ffi::c_void, ptr};

use open62541_sys::{
    UA_AccessControl_default, UA_AccessControl_defaultWithLoginCallback, UA_ByteString,
    UA_ServerConfig, UA_StatusCode, UA_String, UA_UsernamePasswordLogin,
    UA_STATUSCODE_BADINTERNALERROR,
};

use crate::{ua, userdata::UserdataSentinel, DataType, Error, Result, Userdata};

/// Server access control.
///
/// # Safety
///
/// The implementation of [`apply()`] must make sure that existing access control settings in config
/// have been completely replaced or otherwise cleaned up when it returns without an error.
///
/// This is needed to allow dropping sentinel values received from previous calls to [`apply()`] (by
/// different implementations of the trait, possibly) in case [`ServerBuilder::access_control()`] is
/// called twice.
///
/// [`apply()`]: Self::apply
/// [`ServerBuilder::access_control()`]: crate::ServerBuilder::access_control
pub unsafe trait AccessControl {
    /// Sentinel value returned from [`Self::apply()`].
    ///
    /// This allows cleaning up data associated with this instance _after_ the server has shut down,
    /// releasing the access control and not making use of it again.
    type Sentinel: Send + 'static;

    /// Consumes instance and applies it to config.
    ///
    /// # Errors
    ///
    /// This fails when the access control cannot be applied.
    ///
    /// # Safety
    ///
    /// The caller must keep the sentinel value around (not drop it) for as long as the config which
    /// had this access control applied to is still active, i.e. the server has not been shut down.
    unsafe fn apply(self, config: &mut UA_ServerConfig) -> Result<Self::Sentinel>;
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

// SAFETY: `UA_AccessControl_default()` replaces previously set config.
unsafe impl<'a> AccessControl for DefaultAccessControl<'a> {
    type Sentinel = ();

    unsafe fn apply(self, config: &mut UA_ServerConfig) -> Result<Self::Sentinel> {
        let Self {
            allow_anonymous,
            username_password_login,
        } = self;

        let username_password_login = username_password_login
            .iter()
            // SAFETY: `UA_AccessControl_default()` does not take ownership of strings. It uses them
            // only to make internal copies.
            //
            // This also allows the original strings to be dropped once we return from this function
            // (as indicated by lifetime `'a` that may end when `DefaultAccessControl` is dropped).
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

/// Default server access control with login callback.
///
/// This uses `UA_AccessControl_defaultWithLoginCallback()` which comes with the following warning:
///
/// > Example access control management. Anonymous and username/password login. The access rights
/// > are maximally permissive.
/// >
/// > FOR PRODUCTION USE, THIS EXAMPLE PLUGIN SHOULD BE REPLACED WITH LESS PERMISSIVE ACCESS
/// > CONTROL.
/// >
/// > For `TransferSubscriptions`, we check whether the transfer happens between Sessions for the
/// > same user.
#[derive(Debug)]
pub struct DefaultAccessControlWithLoginCallback<F> {
    allow_anonymous: bool,
    login_callback: F,
}

impl<F> DefaultAccessControlWithLoginCallback<F> {
    pub const fn new(allow_anonymous: bool, login_callback: F) -> Self {
        Self {
            allow_anonymous,
            login_callback,
        }
    }
}

// SAFETY: `UA_AccessControl_defaultWithLoginCallback()` replaces previously set config.
unsafe impl<F> AccessControl for DefaultAccessControlWithLoginCallback<F>
where
    // Note the lifetime constraint `'static` here. It is required to prevent accepting closures and
    // moving them into the server config that do not live long enough for the (unknown) lifetime of
    // the `Server` instance that gets eventually built from that config.
    F: Fn(&ua::String, &ua::ByteString) -> ua::StatusCode + Send + 'static,
{
    type Sentinel = UserdataSentinel<F>;

    unsafe fn apply(self, config: &mut UA_ServerConfig) -> Result<Self::Sentinel> {
        unsafe extern "C" fn login_callback_c<F>(
            user_name: *const UA_String,
            password: *const UA_ByteString,
            _username_password_login_size: usize,
            _username_password_login: *const UA_UsernamePasswordLogin,
            _session_context: *mut *mut c_void,
            login_context: *mut c_void,
        ) -> UA_StatusCode
        where
            F: Fn(&ua::String, &ua::ByteString) -> ua::StatusCode,
        {
            let Some(user_name) = (unsafe { user_name.as_ref() }) else {
                return UA_STATUSCODE_BADINTERNALERROR;
            };
            let user_name = ua::String::raw_ref(user_name);

            let Some(password) = (unsafe { password.as_ref() }) else {
                return UA_STATUSCODE_BADINTERNALERROR;
            };
            let password = ua::ByteString::raw_ref(password);

            log::debug!("Handling login request for {user_name:?}");

            let login_callback = unsafe { Userdata::<F>::peek_at(login_context) };

            let status_code = login_callback(user_name, password);

            log::debug!("Login callback for {user_name:?} returned {status_code}");

            // The actual status code is not relevant here: the plugin implementation only looks for
            // `UA_STATUSCODE_GOOD`. Forward other codes directly anyway in case this changes.
            status_code.into_raw()
        }

        let Self {
            allow_anonymous,
            login_callback,
        } = self;

        let username = ua::String::invalid();
        let password = ua::String::invalid();

        // SAFETY: `UA_AccessControl_defaultWithLoginCallback()` does not take ownership of strings,
        // it uses them only to make internal copies. But the strings must only be dropped after the
        // function has returned, so we use the variables above.
        let username_password_login = [unsafe {
            UA_UsernamePasswordLogin {
                username: DataType::to_raw_copy(&username),
                password: DataType::to_raw_copy(&password),
            }
        }];

        let login_callback = Userdata::<F>::prepare(login_callback);

        let status_code = ua::StatusCode::new(unsafe {
            UA_AccessControl_defaultWithLoginCallback(
                config,
                allow_anonymous,
                ptr::null(),
                // The following two arguments would be forwarded to `login_callback_c()`, but we do
                // not make use of them. But we need to set them anyway (to a list with at least one
                // element) for `UA_AccessControl_defaultWithLoginCallback()` to enable the username
                // token policy _at all_.
                username_password_login.len(),
                username_password_login.as_ptr(),
                Some(login_callback_c::<F>),
                login_callback,
            )
        });
        Error::verify_good(&status_code)?;

        // Compile-time assertion to make sure that the strings were still alive at this point.
        drop((username, password));

        // SAFETY: We do not call `consume()` and only create a single sentinel.
        let sentinel = unsafe { Userdata::<F>::sentinel(login_callback) };

        Ok(sentinel)
    }
}
