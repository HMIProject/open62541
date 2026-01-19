use crate::data_types::String;

// [Part 3: 8.4 LocaleId](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.4)
// [Part 5: 12.2.11.1 LocaleId](https://reference.opcfoundation.org/Core/Part5/v105/docs/12.2.11.1)
#[derive(Debug, Clone)]
pub struct LocaleId(pub String);

impl LocaleId {
    pub fn null() -> Self {
        Self(String::null())
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
