// [Part 3: 8.10 ByteString](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.10)
// [Part 6: 5.1.11 Null, Empty and Zero-Length Arrays](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.11)
#[derive(Debug, Clone)]
pub struct ByteString(pub Option<Box<[u8]>>);

impl ByteString {
    pub fn is_null(&self) -> bool {
        self.0.is_none()
    }

    pub fn is_empty(&self) -> bool {
        self.0.as_ref().is_some_and(|value| value.is_empty())
    }
}
