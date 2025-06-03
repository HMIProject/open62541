use open62541_sys::UA_Duration;

use crate::{ua, DataTypeExt};

/// Wrapper for [`UA_Duration`] from [`open62541_sys`].
#[derive(Debug, Clone)]
pub struct Duration(UA_Duration);

const MILLIS_PER_SEC: f64 = 1_000.0;

// See <https://reference.opcfoundation.org/Core/Part3/v105/docs/8.13>.
impl Duration {
    pub(crate) const fn from_f64(mask: f64) -> Self {
        Self(mask)
    }

    /// Creates duration.
    #[must_use]
    pub fn new(value: std::time::Duration) -> Self {
        // OPC UA encodes durations as interval of time in milliseconds.
        Self(value.as_secs_f64() * MILLIS_PER_SEC)
    }

    /// Creates duration from milliseconds.
    ///
    /// The value should be finite. OPC UA does not specify what happens when non-finite values are
    /// used for the underlying `Double` value.
    ///
    /// Negative values are generally invalid but may have special meanings where the `Duration` is
    /// used.
    #[must_use]
    pub const fn from_millis(value: f64) -> Self {
        Self(value)
    }

    pub(crate) const fn as_f64(&self) -> f64 {
        self.0
    }

    /// Gets duration in milliseconds.
    ///
    /// This returns the underlying raw value, which may be negative. Negative values are generally
    /// invalid but may have special meanings where the `Duration` is used.
    #[must_use]
    pub const fn as_millis(&self) -> f64 {
        self.0
    }

    /// Gets duration value.
    ///
    /// This returns `None` when the underlying number of milliseconds is negative. Negative values
    /// are generally invalid but may have special meanings where the `Duration` is used.
    ///
    /// Use [`Self::as_millis()`] to get the raw value.
    #[must_use]
    pub fn to_duration(&self) -> Option<std::time::Duration> {
        // OPC UA encodes durations as interval of time in milliseconds.
        std::time::Duration::try_from_secs_f64(self.as_f64() / MILLIS_PER_SEC).ok()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_durations() {
        let duration = Duration::new(std::time::Duration::from_millis(1_125));
        assert_eq!(duration.as_millis(), 1_125.0);
        assert_eq!(
            duration.to_duration(),
            Some(std::time::Duration::from_secs_f32(1.125))
        );
    }

    #[test]
    fn it_handles_negative_values() {
        let duration = Duration::from_millis(-1_125.0);
        assert_eq!(duration.as_millis(), -1_125.0);
        assert_eq!(duration.to_duration(), None);
    }
}
