use std::str::FromStr;

use crate::{ua, DataType as _};

crate::data_type!(ApplicationDescription);

/// Typically, `ApplicationDescription` is generated from static data. Therefore, the methods below
/// do not return `Result` but panic instead when the given strings are invalid (when they contain
/// NUL bytes).
impl ApplicationDescription {
    #[must_use]
    pub fn with_application_uri(mut self, application_uri: &str) -> Self {
        ua::String::from_str(application_uri)
            .unwrap()
            .move_into_raw(&mut self.0.applicationUri);
        self
    }

    #[must_use]
    pub fn with_product_uri(mut self, product_uri: &str) -> Self {
        ua::String::from_str(product_uri)
            .unwrap()
            .move_into_raw(&mut self.0.productUri);
        self
    }

    #[must_use]
    pub fn with_application_name(mut self, locale: &str, application_name: &str) -> Self {
        ua::LocalizedText::try_from((locale, application_name))
            .unwrap()
            .move_into_raw(&mut self.0.applicationName);
        self
    }

    #[must_use]
    pub fn with_application_type(mut self, application_type: ua::ApplicationType) -> Self {
        application_type.move_into_raw(&mut self.0.applicationType);
        self
    }

    #[must_use]
    pub fn with_gateway_server_uri(mut self, gateway_server_uri: &str) -> Self {
        ua::String::from_str(gateway_server_uri)
            .unwrap()
            .move_into_raw(&mut self.0.gatewayServerUri);
        self
    }

    #[must_use]
    pub fn with_discovery_profile_uri(mut self, discovery_profile_uri: &str) -> Self {
        ua::String::from_str(discovery_profile_uri)
            .unwrap()
            .move_into_raw(&mut self.0.discoveryProfileUri);
        self
    }

    #[must_use]
    pub fn with_discovery_urls(mut self, discovery_urls: &[&str]) -> Self {
        let discovery_urls = discovery_urls
            .iter()
            .map(|discovery_url| ua::String::from_str(discovery_url).unwrap());
        ua::Array::from_iter(discovery_urls)
            .move_into_raw(&mut self.0.discoveryUrlsSize, &mut self.0.discoveryUrls);
        self
    }
}
