use open62541_sys::UA_NumericRangeDimension;

/// Wrapper for [`UA_NumericRangeDimension`] from [`open62541_sys`].
#[repr(transparent)]
pub struct NumericRangeDimension(UA_NumericRangeDimension);

impl NumericRangeDimension {
    #[must_use]
    pub const fn min(&self) -> u32 {
        self.0.min
    }

    #[must_use]
    pub const fn max(&self) -> u32 {
        self.0.max
    }
}
