use std::string::String as StdString;

pub use self::locale_id::LocaleId;

// [Part 3: 8.31 String](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.31)
// [Part 5: 12.2.11 String](https://reference.opcfoundation.org/Core/Part5/v105/docs/12.2.11)
// [Part 6: 5.1.11 Null, Empty and Zero-Length Arrays](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.11)
pub struct String(pub Option<StdString>);

impl String {
    pub fn null() -> Self {
        Self(None)
    }

    pub fn is_null(&self) -> bool {
        self.0.is_none()
    }

    pub fn is_empty(&self) -> bool {
        self.0.as_ref().is_some_and(|value| value.is_empty())
    }
}

mod locale_id {
    use super::String;

    // [Part 3: 8.4 LocaleId](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.4)
    // [Part 5: 12.2.11.1 LocaleId](https://reference.opcfoundation.org/Core/Part5/v105/docs/12.2.11.1)
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
}
