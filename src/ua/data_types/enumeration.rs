crate::data_type!(Enumeration);

impl Enumeration {
    /// Extracts raw enum value.
    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "unrelated technicality")]
    pub fn as_u32(&self) -> u32 {
        // This cast is necessary on Windows builds with inner type `i32`.
        #[allow(clippy::useless_conversion, reason = "bindgen i32")]
        u32::try_from((self.0).0).expect("should convert to u32")
    }
}
