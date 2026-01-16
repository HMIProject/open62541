use super::{LocaleId, String};

// [Part 3: 8.5 LocalizedText](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.5)
pub struct LocalizedText(pub LocaleId, pub String);
