use crate::{ua, DataType as _};

crate::data_type!(IssuedIdentityToken);

impl IssuedIdentityToken {
    /// Sets policy ID.
    pub fn with_policy_id(mut self, policy_id: ua::String) -> Self {
        policy_id.move_into_raw(&mut self.0.policyId);
        self
    }

    /// Sets token data.
    pub fn with_token_data(mut self, token_data: ua::ByteString) -> Self {
        token_data.move_into_raw(&mut self.0.tokenData);
        self
    }

    /// Sets encryption algorithm.
    pub fn with_encryption_algorithm(mut self, encryption_algorithm: ua::String) -> Self {
        encryption_algorithm.move_into_raw(&mut self.0.encryptionAlgorithm);
        self
    }
}
