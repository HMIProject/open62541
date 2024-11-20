use std::{fmt, mem::MaybeUninit};

use open62541_sys::{UA_ClientConfig, UA_ClientConfig_clear, UA_ClientConfig_setDefault};

use crate::{ua, Error};

pub(crate) struct ClientConfig(Option<UA_ClientConfig>);

impl ClientConfig {
    #[must_use]
    fn new() -> Self {
        let mut config = Self::init();

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
            config.logging = crate::logger();
        }

        // Next, we must finish initialization by calling `UA_ClientConfig_set...()` as appropriate.
        // This happens in the caller.
        config
    }

    /// Creates default client config.
    // Method name refers to call of `UA_ClientConfig_setDefault()`.
    #[must_use]
    pub(crate) fn default() -> Self {
        let mut config = Self::new();

        // Set remaining attributes to their desired values. This also copies the logger as laid out
        // above to other attributes inside `config` (cleaned up by `UA_ClientConfig_clear()`).
        let status_code =
            ua::StatusCode::new(unsafe { UA_ClientConfig_setDefault(config.as_mut_ptr()) });
        // PANIC: The only possible errors here are out-of-memory.
        Error::verify_good(&status_code).expect("should set default client config");

        config
    }

    /// Creates default client config with encryption.
    // Method name refers to call of `UA_ClientConfig_setDefaultEncryption()`.
    #[cfg(feature = "mbedtls")]
    pub(crate) fn default_encryption(
        certificate: &[u8],
        private_key: &[u8],
    ) -> Result<Self, crate::Error> {
        use {crate::DataType, open62541_sys::UA_ClientConfig_setDefaultEncryption, std::ptr};

        let mut config = Self::new();

        // Set remaining attributes to their desired values. This also copies the logger as laid out
        // above to other attributes inside `config` (cleaned up by `UA_ClientConfig_clear()`).
        let status_code = ua::StatusCode::new(unsafe {
            UA_ClientConfig_setDefaultEncryption(
                config.as_mut_ptr(),
                // SAFETY: Function expects struct instead of pointer, despite not taking ownership.
                DataType::to_raw_copy(&ua::ByteString::new(certificate)),
                DataType::to_raw_copy(&ua::ByteString::new(private_key)),
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
    pub(crate) const unsafe fn from_raw(src: UA_ClientConfig) -> Self {
        Self(Some(src))
    }

    /// Gives up ownership and returns value.
    ///
    /// The returned value must be re-wrapped with [`from_raw()`], cleared manually, or copied into
    /// an owning value (like [`UA_Client`]) to free internal allocations and not leak memory.
    ///
    /// [`from_raw()`]: Self::from_raw
    /// [`UA_Client`]: open62541_sys::UA_Client
    #[must_use]
    pub(crate) fn into_raw(mut self) -> UA_ClientConfig {
        self.0.take().expect("should have client config")
    }

    /// Creates wrapper initialized with defaults.
    ///
    /// This initializes the value and makes all attributes well-defined. Additional attributes may
    /// need to be initialized for the value to be actually useful afterwards.
    pub(crate) const fn init() -> Self {
        let inner = MaybeUninit::<UA_ClientConfig>::zeroed();
        // SAFETY: Zero-initialized memory is a valid client config.
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
    pub(crate) unsafe fn as_mut(&mut self) -> &mut UA_ClientConfig {
        // PANIC: The inner object can only be unset when ownership has been given away.
        self.0.as_mut().expect("should have client config")
    }

    /// Returns mutable pointer to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[must_use]
    pub(crate) unsafe fn as_mut_ptr(&mut self) -> *mut UA_ClientConfig {
        // PANIC: The inner object can only be unset when ownership has been given away.
        self.0.as_mut().expect("should have client config")
    }
}

impl Drop for ClientConfig {
    fn drop(&mut self) {
        // Check if we still hold the client config. If not, we need not clean up: the ownership has
        // passed to the client that was created from this config.
        if let Some(mut inner) = self.0.take() {
            unsafe { UA_ClientConfig_clear(&mut inner) }
        }
    }
}

impl fmt::Debug for ClientConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClientConfig").finish_non_exhaustive()
    }
}
