use crate::{DataType as _, ua};

crate::data_type!(HistoryReadResult);

impl HistoryReadResult {
    #[must_use]
    pub fn status(&self) -> &ua::StatusCode {
        ua::StatusCode::raw_ref(&self.0.statusCode)
    }

    #[must_use]
    pub fn continuation_point(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.continuationPoint)
    }

    #[must_use]
    pub fn history_data(&mut self) -> &ua::ExtensionObject {
        ua::ExtensionObject::raw_ref(&self.0.historyData)
    }
}
