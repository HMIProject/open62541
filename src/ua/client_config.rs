use std::{ffi::c_void, fmt, mem::MaybeUninit};

use open62541_sys::{UA_ClientConfig, UA_ClientConfig_clear, UA_ClientConfig_setDefault};

use crate::{ua, ClientContext, Error};

pub(crate) struct ClientConfig(Option<UA_ClientConfig>);

impl ClientConfig {
    #[must_use]
    fn new(context: ClientContext) -> Self {
        #[cfg(feature = "mbedtls")]
        unsafe extern "C" fn private_key_password_callback_c(
            cc: *mut UA_ClientConfig,
            password: *mut open62541_sys::UA_ByteString,
        ) -> open62541_sys::UA_StatusCode {
            use crate::DataType as _;

            // Unwrap incoming arguments. This should always work: `password` is a valid instance of
            // `UA_ByteString` even though it is only used as out argument.
            let Some(cc) = (unsafe { cc.as_mut() }) else {
                return ua::StatusCode::BADINTERNALERROR.into_raw();
            };
            let Some(password) = (unsafe { password.as_mut() }) else {
                return ua::StatusCode::BADINTERNALERROR.into_raw();
            };

            // We always expect to find the context as initialized by `Self::new()` further below.
            let Some(context) = (unsafe { cc.clientContext.cast::<ClientContext>().as_mut() })
            else {
                return ua::StatusCode::BADINTERNALERROR.into_raw();
            };

            // The callback is only set by `ClientBuilder::private_key_password_callback()`.
            let Some(private_key_password_callback) =
                context.private_key_password_callback.as_ref()
            else {
                return ua::StatusCode::BADCONFIGURATIONERROR.into_raw();
            };

            let status_code = match private_key_password_callback.private_key_password() {
                Ok(result_password) => {
                    // This clones the password string. The original string from the callback method
                    // is zeroized. The cloned string is handled by `open62541`, eventually zeroized
                    // there as well in implementation of `UA_ClientConfig_setDefaultEncryption()`.
                    result_password.as_byte_string().clone_into_raw(password);
                    ua::StatusCode::GOOD
                }
                Err(err) => err.status_code(),
            };

            status_code.into_raw()
        }

        let mut config = Self::init();

        // Initialize default attributes. None of these must return early to avoid leaking memory in
        // case the config has only been half-initialized.
        //
        // We set the custom logger here before the caller calls `UA_ClientConfig_set...()`. This is
        // necessary because the same logger instance will be used inside derived attributes such as
        // `eventLoop`, `certificateVerification`, etc.
        {
            let config = unsafe { config.as_mut() };

            debug_assert!(config.clientContext.is_null());
            // Set custom client context. This leaks memory which is later reclaimed when the config
            // or the client that was created from it is dropped.
            config.clientContext = Box::into_raw(Box::new(context)).cast::<c_void>();

            // We assign a logger only on default-initialized config objects: we cannot know whether
            // an existing configuration is still referenced in another attribute or structure, thus
            // we could not (safely) free it anyway.
            debug_assert!(config.logging.is_null());
            // Create logger configuration. Ownership of the `UA_Logger` instance passes to `config`
            // at this point.
            config.logging = ua::Logger::rust_log().into_raw();

            // Initialize callback for fetching private-key password when compiling for SSL support.
            // We always set this, even when `ClientBuilder::private_key_password_callback()` is not
            // called, to skip the default implementation where `open62541` requests the password on
            // the terminal (stdin), issuing an unexpected, blocking prompt.
            #[cfg(feature = "mbedtls")]
            {
                debug_assert!(config.privateKeyPasswordCallback.is_none());
                config.privateKeyPasswordCallback = Some(private_key_password_callback_c);
            }
        }

        // Next, we must finish initialization by calling `UA_ClientConfig_set...()` as appropriate.
        // This happens in the caller.
        config
    }

    /// Creates default client config.
    // Method name refers to call of `UA_ClientConfig_setDefault()`.
    #[must_use]
    pub(crate) fn default(context: ClientContext) -> Self {
        let mut config = Self::new(context);

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
        context: ClientContext,
        local_certificate: &crate::Certificate,
        private_key: &crate::PrivateKey,
    ) -> Result<Self, crate::Error> {
        use {crate::DataType, open62541_sys::UA_ClientConfig_setDefaultEncryption, std::ptr};

        let mut config = Self::new(context);

        // Set remaining attributes to their desired values. This also copies the logger as laid out
        // above to other attributes inside `config` (cleaned up by `UA_ClientConfig_clear()`).
        let status_code = ua::StatusCode::new(unsafe {
            UA_ClientConfig_setDefaultEncryption(
                config.as_mut_ptr(),
                // SAFETY: Function expects struct instead of pointer, despite not taking ownership.
                DataType::to_raw_copy(local_certificate.as_byte_string()),
                DataType::to_raw_copy(private_key.as_byte_string()),
                ptr::null(),
                0,
                ptr::null(),
                0,
            )
        });
        Error::verify_good(&status_code)?;

        Ok(config)
    }

    /// Access client context.
    #[must_use]
    pub(crate) fn context_mut(&mut self) -> &mut ClientContext {
        let context = unsafe { self.as_mut() }
            .clientContext
            .cast::<ClientContext>();
        // SAFETY: `clientContext` attribute is of the expected type.
        unsafe { context.as_mut() }.expect("client context must be set")
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
    #[must_use]
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
    #[expect(clippy::allow_attributes, reason = "non-static condition")]
    #[allow(clippy::missing_const_for_fn, reason = "unsupported before Rust 1.87")]
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
    #[expect(clippy::allow_attributes, reason = "non-static condition")]
    #[allow(clippy::missing_const_for_fn, reason = "unsupported before Rust 1.87")]
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
            // Fetch context pointer before clearing config. Free associated memory only afterwards.
            let context = inner.clientContext.cast::<ClientContext>();

            unsafe { UA_ClientConfig_clear(&raw mut inner) }

            // Reclaim wrapped client context to avoid leaking memory. This simply drops the value.
            let _context: Box<ClientContext> = unsafe { Box::from_raw(context) };
        }
    }
}

impl fmt::Debug for ClientConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClientConfig").finish_non_exhaustive()
    }
}
