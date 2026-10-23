use crate::ua;

crate::data_type!(UserTokenPolicy);
crate::member_accessors!(UserTokenPolicy {
    policyId: &ua::String,
    #[enum]
    tokenType: ua::UserTokenType,
    issuedTokenType: &ua::String,
    issuerEndpointUrl: &ua::String,
    securityPolicyUri: &ua::String
});
