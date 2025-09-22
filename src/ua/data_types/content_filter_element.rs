use crate::{DataType, FilterOperand, ua};

crate::data_type!(ContentFilterElement);

impl ContentFilterElement {
    #[must_use]
    pub fn with_filter_operator(mut self, filter_operator: ua::FilterOperator) -> Self {
        filter_operator.move_into_raw(&mut self.0.filterOperator);
        self
    }

    #[must_use]
    pub fn with_filter_operands(mut self, filter_operands: &[impl FilterOperand]) -> Self {
        let array = ua::Array::from_iter(
            filter_operands
                .iter()
                .map(FilterOperand::to_extension_object),
        );
        array.move_into_raw(&mut self.0.filterOperandsSize, &mut self.0.filterOperands);
        self
    }
}
