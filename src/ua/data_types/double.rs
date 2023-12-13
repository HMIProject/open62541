crate::data_type!(Double, UA_Double, UA_TYPES_DOUBLE);

impl Double {
    #[must_use]
    pub const fn value(value: f64) -> Self {
        Self::new(value)
    }
}
