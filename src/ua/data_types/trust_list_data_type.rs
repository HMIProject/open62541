use crate::ua;

crate::data_type!(TrustListDataType);

impl TrustListDataType {
    #[must_use]
    pub fn with_trusted_certificates(mut self, trusted_certificates: &[ua::String]) -> Self {
        let array = ua::Array::from_slice(trusted_certificates);
        if array.is_empty() {
            self.0.specifiedLists &= !ua::TrustListMasks::TRUSTEDCERTIFICATES.as_u32();
        } else {
            self.0.specifiedLists |= ua::TrustListMasks::TRUSTEDCERTIFICATES.as_u32();
        }
        array.move_into_raw(
            &mut self.0.trustedCertificatesSize,
            &mut self.0.trustedCertificates,
        );
        self
    }

    #[must_use]
    pub fn with_trusted_crls(mut self, trusted_crls: &[ua::String]) -> Self {
        let array = ua::Array::from_slice(trusted_crls);
        if array.is_empty() {
            self.0.specifiedLists &= !ua::TrustListMasks::TRUSTEDCRLS.as_u32();
        } else {
            self.0.specifiedLists |= ua::TrustListMasks::TRUSTEDCRLS.as_u32();
        }
        array.move_into_raw(&mut self.0.trustedCrlsSize, &mut self.0.trustedCrls);
        self
    }

    #[must_use]
    pub fn with_issuer_certificates(mut self, issuer_certificates: &[ua::String]) -> Self {
        let array = ua::Array::from_slice(issuer_certificates);
        if array.is_empty() {
            self.0.specifiedLists &= !ua::TrustListMasks::ISSUERCERTIFICATES.as_u32();
        } else {
            self.0.specifiedLists |= ua::TrustListMasks::ISSUERCERTIFICATES.as_u32();
        }
        array.move_into_raw(
            &mut self.0.issuerCertificatesSize,
            &mut self.0.issuerCertificates,
        );
        self
    }

    #[must_use]
    pub fn with_issuer_crls(mut self, issuer_crls: &[ua::String]) -> Self {
        let array = ua::Array::from_slice(issuer_crls);
        if array.is_empty() {
            self.0.specifiedLists &= !ua::TrustListMasks::ISSUERCRLS.as_u32();
        } else {
            self.0.specifiedLists |= ua::TrustListMasks::ISSUERCRLS.as_u32();
        }
        array.move_into_raw(&mut self.0.issuerCrlsSize, &mut self.0.issuerCrls);
        self
    }
}
