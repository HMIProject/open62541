crate::data_type!(Range);

impl Range {
    #[must_use]
    pub const fn low(&self) -> f64 {
        self.0.low
    }

    #[must_use]
    pub const fn high(&self) -> f64 {
        self.0.high
    }
}
