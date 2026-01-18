use super::{StatusCode, String};

// [Part 4: 7.12 DiagnosticInfo](https://reference.opcfoundation.org/Core/Part4/v105/docs/7.12)
// [Part 6: 5.2.2.12 DiagnosticInfo](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.12)
pub struct DiagnosticInfo {
    pub namespace_uri: Option<i32>,
    pub symbolic_id: Option<i32>,
    pub locale: Option<i32>,
    pub localized_text: Option<i32>,
    pub additional_info: Option<String>,
    pub inner_status_code: Option<StatusCode>,
    pub inner_diagnostic_info: Option<Box<DiagnosticInfo>>,
}
