use crate::{ua, DataType as _};

crate::data_type!(AnonymousIdentityToken);

impl AnonymousIdentityToken {
    /// Sets policy ID.
    #[must_use]
    pub fn with_policy_id(mut self, policy_id: ua::String) -> Self {
        policy_id.move_into_raw(&mut self.0.policyId);
        self
    }
}
