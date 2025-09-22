use std::{fmt, mem::MaybeUninit, ptr};

use open62541_sys::{UA_ServerConfig, UA_ServerConfig_clean, UA_ServerConfig_setMinimal};

use crate::{DataType as _, Error, ua};

pub(crate) struct ServerConfig(Option<UA_ServerConfig>);

impl ServerConfig {
    #[must_use]
    fn new() -> Self {
        let mut config = Self::init();

        let logger = ua::Logger::rust_log();

        // Set custom logger first. This is necessary because the same logger instance is used as-is
        // inside derived attributes such as `eventLoop`, `certificateVerification`, etc.
        {
            let config = unsafe { config.as_mut() };
            // We assign a logger only on default-initialized config objects: we cannot know whether
            // an existing configuration is still referenced in another attribute or structure, thus
            // we could not (safely) free it anyway.
            debug_assert!(config.logging.is_null());
            // Create logger configuration. Ownership of the `UA_Logger` instance passes to `config`
            // at this point.
            config.logging = logger.into_raw();
        }

        // Next, we must finish initialization by calling `UA_ServerConfig_set...()` as appropriate.
        // This happens in the caller.
        config
    }

    /// Creates minimal server config.
    // Method name refers to call of `UA_ServerConfig_setMinimal()`.
    #[must_use]
    pub(crate) fn minimal(port_number: u16, certificate: Option<&[u8]>) -> Self {
        let mut config = Self::new();

        // Set remaining attributes to their desired values. This also copies the logger as laid out
        // above to other attributes inside `config` (cleaned up by `UA_ServerConfig_clean()`).
        let status_code = ua::StatusCode::new(unsafe {
            UA_ServerConfig_setMinimal(
                config.as_mut_ptr(),
                port_number,
                certificate.map_or(ptr::null(), |certificate| {
                    ua::ByteString::new(certificate).as_ptr()
                }),
            )
        });
        // PANIC: The only possible errors here are out-of-memory.
        Error::verify_good(&status_code).expect("should set minimal server config");

        config
    }

    /// Creates a default server config with security policies.
    // Method name refers to call of `UA_ServerConfig_setDefaultWithSecurityPolicies()`.
    #[cfg(feature = "mbedtls")]
    pub(crate) fn default_with_security_policies(
        port_number: u16,
        certificate: &crate::Certificate,
        private_key: &crate::PrivateKey,
    ) -> Result<Self, crate::Error> {
        use open62541_sys::UA_ServerConfig_setDefaultWithSecurityPolicies;

        let mut config = Self::new();

        // Set remaining attributes to their desired values. This also copies the logger as laid out
        // above to other attributes inside `config` (cleaned up by `UA_ServerConfig_clean()`).
        let status_code = ua::StatusCode::new(unsafe {
            UA_ServerConfig_setDefaultWithSecurityPolicies(
                config.as_mut_ptr(),
                port_number,
                certificate.as_byte_string().as_ptr(),
                private_key.as_byte_string().as_ptr(),
                ptr::null(),
                0,
                ptr::null(),
                0,
                ptr::null(),
                0,
            )
        });
        Error::verify_good(&status_code)?;

        Ok(config)
    }

    /// Creates a default server config with secure security policies.
    // Method name refers to call of `UA_ServerConfig_setDefaultWithSecureSecurityPolicies()`.
    #[cfg(feature = "mbedtls")]
    pub(crate) fn default_with_secure_security_policies(
        port_number: u16,
        certificate: &[u8],
        private_key: &[u8],
    ) -> Result<Self, crate::Error> {
        use open62541_sys::UA_ServerConfig_setDefaultWithSecureSecurityPolicies;

        let mut config = Self::new();

        // Set remaining attributes to their desired values. This also copies the logger as laid out
        // above to other attributes inside `config` (cleaned up by `UA_ServerConfig_clean()`).
        let status_code = ua::StatusCode::new(unsafe {
            UA_ServerConfig_setDefaultWithSecureSecurityPolicies(
                config.as_mut_ptr(),
                port_number,
                ua::ByteString::new(certificate).as_ptr(),
                ua::ByteString::new(private_key).as_ptr(),
                ptr::null(),
                0,
                ptr::null(),
                0,
                ptr::null(),
                0,
            )
        });
        Error::verify_good(&status_code)?;

        Ok(config)
    }

    /// Creates wrapper by taking ownership of value.
    ///
    /// When `Self` is dropped, allocations held by the inner type are cleaned up.
    ///
    /// # Safety
    ///
    /// Ownership of the value passes to `Self`. This must only be used for values that are not
    /// contained within other values that may be dropped.
    #[must_use]
    pub(crate) const unsafe fn from_raw(src: UA_ServerConfig) -> Self {
        Self(Some(src))
    }

    /// Gives up ownership and returns value.
    ///
    /// The returned value must be re-wrapped with [`from_raw()`], cleared manually, or copied into
    /// an owning value (like [`UA_Server`]) to free internal allocations and not leak memory.
    ///
    /// [`from_raw()`]: Self::from_raw
    /// [`UA_Server`]: open62541_sys::UA_Server
    #[must_use]
    pub(crate) fn into_raw(mut self) -> UA_ServerConfig {
        self.0.take().expect("should have server config")
    }

    /// Creates wrapper initialized with defaults.
    ///
    /// This initializes the value and makes all attributes well-defined. Additional attributes may
    /// need to be initialized for the value to be actually useful afterwards.
    pub(crate) const fn init() -> Self {
        let inner = MaybeUninit::<UA_ServerConfig>::zeroed();
        // SAFETY: Zero-initialized memory is a valid server config.
        let inner = unsafe { inner.assume_init() };
        // SAFETY: We pass a value without pointers to it into `Self`.
        unsafe { Self::from_raw(inner) }
    }

    /// Returns exclusive reference to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[must_use]
    #[expect(clippy::allow_attributes, reason = "non-static condition")]
    #[allow(clippy::missing_const_for_fn, reason = "unsupported before Rust 1.87")]
    pub(crate) unsafe fn as_mut(&mut self) -> &mut UA_ServerConfig {
        // PANIC: The inner object can only be unset when ownership has been given away.
        self.0.as_mut().expect("should have server config")
    }

    /// Returns mutable pointer to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[must_use]
    #[expect(clippy::allow_attributes, reason = "non-static condition")]
    #[allow(clippy::missing_const_for_fn, reason = "unsupported before Rust 1.87")]
    pub(crate) unsafe fn as_mut_ptr(&mut self) -> *mut UA_ServerConfig {
        // PANIC: The inner object can only be unset when ownership has been given away.
        self.0.as_mut().expect("should have server config")
    }
}

impl Drop for ServerConfig {
    fn drop(&mut self) {
        // Check if we still hold the server config. If not, we need not clean up: the ownership has
        // passed to the server that was created from this config.
        if let Some(mut inner) = self.0.take() {
            unsafe { UA_ServerConfig_clean(&raw mut inner) }
        }
    }
}

impl fmt::Debug for ServerConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServerConfig").finish_non_exhaustive()
    }
}
