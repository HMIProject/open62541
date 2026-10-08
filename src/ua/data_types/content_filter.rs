use crate::ua;

crate::data_type!(ContentFilter);
crate::member_accessors!(ContentFilter {
    elements: [ua::ContentFilterElement]
});

impl ContentFilter {
    #[must_use]
    pub fn with_elements(mut self, elements: &[ua::ContentFilterElement]) -> Self {
        let array = ua::Array::from_slice(elements);
        array.move_into_raw(&mut self.0.elementsSize, &mut self.0.elements);
        self
    }
}
