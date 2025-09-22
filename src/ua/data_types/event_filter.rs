use crate::{DataType as _, MonitoringFilter, ua};

crate::data_type!(EventFilter);

impl EventFilter {
    #[must_use]
    pub fn with_select_clauses(mut self, select_clauses: &[ua::SimpleAttributeOperand]) -> Self {
        let array = ua::Array::from_slice(select_clauses);
        array.move_into_raw(&mut self.0.selectClausesSize, &mut self.0.selectClauses);
        self
    }

    #[must_use]
    pub fn with_where_clause(mut self, where_clause: ua::ContentFilter) -> Self {
        where_clause.move_into_raw(&mut self.0.whereClause);
        self
    }
}

impl MonitoringFilter for EventFilter {
    fn to_extension_object(&self) -> ua::ExtensionObject {
        ua::ExtensionObject::new(self)
    }
}
