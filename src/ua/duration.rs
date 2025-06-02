use open62541_sys::UA_Duration;

use crate::{ua, DataTypeExt};

/// Wrapper for [`UA_Duration`] from [`open62541_sys`].
#[derive(Debug, Clone)]
pub struct Duration(UA_Duration);

// See <https://reference.opcfoundation.org/Core/Part3/v105/docs/8.13>.
impl Duration {
    pub(crate) const fn from_f64(mask: f64) -> Self {
        Self(mask)
    }

    pub(crate) const fn as_f64(&self) -> f64 {
        self.0
    }

    #[must_use]
    pub fn to_duration(&self) -> Option<std::time::Duration> {
        // OPC UA gives durations as interval of time in milliseconds.
        std::time::Duration::try_from_secs_f64(self.as_f64() / 1e3).ok()
    }
}

impl DataTypeExt for Duration {
    type Inner = ua::Double;

    fn from_inner(value: Self::Inner) -> Self {
        Self::from_f64(value.value())
    }

    fn into_inner(self) -> Self::Inner {
        Self::Inner::new(self.as_f64())
    }
}
