use super::{StatusCode, String};

// [Part 4: 7.12 DiagnosticInfo](https://reference.opcfoundation.org/Core/Part4/v105/docs/7.12)
// [Part 6: 5.2.2.12 DiagnosticInfo](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.12)
pub struct DiagnosticInfo {
    namespace_uri: Option<i32>,
    symbolic_id: Option<i32>,
    locale: Option<i32>,
    localized_text: Option<i32>,
    additional_info: Option<String>,
    inner_status_code: Option<StatusCode>,
    inner_diagnostic_info: Option<Box<DiagnosticInfo>>,
}
