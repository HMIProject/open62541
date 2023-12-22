use open62541_sys::UA_TimestampsToReturn;

crate::data_type!(TimestampsToReturn);

impl TimestampsToReturn {
    #[must_use]
    pub const fn source() -> Self {
        Self(UA_TimestampsToReturn::UA_TIMESTAMPSTORETURN_SOURCE)
    }

    #[must_use]
    pub const fn server() -> Self {
        Self(UA_TimestampsToReturn::UA_TIMESTAMPSTORETURN_SERVER)
    }

    #[must_use]
    pub const fn both() -> Self {
        Self(UA_TimestampsToReturn::UA_TIMESTAMPSTORETURN_BOTH)
    }

    #[must_use]
    pub const fn neither() -> Self {
        Self(UA_TimestampsToReturn::UA_TIMESTAMPSTORETURN_NEITHER)
    }

    #[must_use]
    pub const fn invalid() -> Self {
        Self(UA_TimestampsToReturn::UA_TIMESTAMPSTORETURN_INVALID)
    }
}
