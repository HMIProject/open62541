use crate::{ua, DataType as _};

crate::data_type!(ApplicationDescription);

/// Typically, `ApplicationDescription` is generated from static data. Therefore, the methods below
/// do not return `Result` but panic instead when the given strings are invalid (when they contain
/// NUL bytes).
impl ApplicationDescription {
    /// Sets application URI.
    ///
    /// # Panics
    ///
    /// The string must not contain any NUL bytes.
    #[must_use]
    pub fn with_application_uri(mut self, application_uri: &str) -> Self {
        ua::String::new(application_uri)
            .unwrap()
            .move_into_raw(&mut self.0.applicationUri);
        self
    }

    /// Sets product URI.
    ///
    /// # Panics
    ///
    /// The string must not contain any NUL bytes.
    #[must_use]
    pub fn with_product_uri(mut self, product_uri: &str) -> Self {
        ua::String::new(product_uri)
            .unwrap()
            .move_into_raw(&mut self.0.productUri);
        self
    }

    /// Sets application name (with locale).
    ///
    /// # Panics
    ///
    /// The strings must not contain any NUL bytes.
    #[must_use]
    pub fn with_application_name(mut self, locale: &str, application_name: &str) -> Self {
        ua::LocalizedText::new(locale, application_name)
            .unwrap()
            .move_into_raw(&mut self.0.applicationName);
        self
    }

    /// Sets application type.
    ///
    #[must_use]
    pub fn with_application_type(mut self, application_type: ua::ApplicationType) -> Self {
        application_type.move_into_raw(&mut self.0.applicationType);
        self
    }

    /// Sets gateway server URI.
    ///
    /// # Panics
    ///
    /// The string must not contain any NUL bytes.
    #[must_use]
    pub fn with_gateway_server_uri(mut self, gateway_server_uri: &str) -> Self {
        ua::String::new(gateway_server_uri)
            .unwrap()
            .move_into_raw(&mut self.0.gatewayServerUri);
        self
    }

    /// Sets discovery profile URI.
    ///
    /// # Panics
    ///
    /// The string must not contain any NUL bytes.
    #[must_use]
    pub fn with_discovery_profile_uri(mut self, discovery_profile_uri: &str) -> Self {
        ua::String::new(discovery_profile_uri)
            .unwrap()
            .move_into_raw(&mut self.0.discoveryProfileUri);
        self
    }

    /// Sets discovery URLs.
    ///
    /// # Panics
    ///
    /// The strings must not contain any NUL bytes.
    #[must_use]
    pub fn with_discovery_urls(mut self, discovery_urls: &[&str]) -> Self {
        let discovery_urls = discovery_urls
            .iter()
            .map(|discovery_url| ua::String::new(discovery_url).unwrap());
        ua::Array::from_iter(discovery_urls)
            .move_into_raw(&mut self.0.discoveryUrlsSize, &mut self.0.discoveryUrls);
        self
    }

    #[must_use]
    pub fn application_uri(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.applicationUri)
    }

    #[must_use]
    pub fn product_uri(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.productUri)
    }

    #[must_use]
    pub fn application_name(&self) -> &ua::LocalizedText {
        ua::LocalizedText::raw_ref(&self.0.applicationName)
    }

    #[must_use]
    pub fn application_type(&self) -> &ua::ApplicationType {
        ua::ApplicationType::raw_ref(&self.0.applicationType)
    }

    #[must_use]
    pub fn gateway_server_uri(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.gatewayServerUri)
    }

    #[must_use]
    pub fn discovery_profile_uri(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.discoveryProfileUri)
    }

    #[must_use]
    pub fn discovery_urls(&self) -> Option<&[ua::String]> {
        unsafe { ua::Array::slice_from_raw_parts(self.0.discoveryUrlsSize, self.0.discoveryUrls) }
    }
}
