use crate::ua;

crate::data_type!(TrustListDataType);

impl TrustListDataType {
    #[must_use]
    pub fn with_trusted_certificates(mut self, trusted_certificates: &[ua::String]) -> Self {
        let array = ua::Array::from_slice(trusted_certificates);
        array.move_into_raw(
            &mut self.0.trustedCertificatesSize,
            &mut self.0.trustedCertificates,
        );
        // FIXME: How to set masks?
        // self.0.specifiedLists |= UA_TRUSTLISTMASKS_TRUSTEDCERTIFICATES;
        self
    }

    #[must_use]
    pub fn with_trusted_crls(mut self, trusted_crls: &[ua::String]) -> Self {
        let array = ua::Array::from_slice(trusted_crls);
        array.move_into_raw(&mut self.0.trustedCrlsSize, &mut self.0.trustedCrls);
        // FIXME: How to set masks?
        self
    }

    #[must_use]
    pub fn with_issuer_certificates(mut self, issuer_certificates: &[ua::String]) -> Self {
        let array = ua::Array::from_slice(issuer_certificates);
        array.move_into_raw(
            &mut self.0.issuerCertificatesSize,
            &mut self.0.issuerCertificates,
        );
        // FIXME: How to set masks?
        self
    }

    #[must_use]
    pub fn with_issuer_crls(mut self, issuer_crls: &[ua::String]) -> Self {
        let array = ua::Array::from_slice(issuer_crls);
        array.move_into_raw(&mut self.0.issuerCrlsSize, &mut self.0.issuerCrls);
        // FIXME: How to set masks?
        self
    }
}
