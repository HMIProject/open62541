// [Part 5: 12.3.14 StatusCode](https://reference.opcfoundation.org/Core/Part5/v105/docs/12.3.14)
// [Part 6: 5.2.2.11 StatusCode](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.11)
pub struct StatusCode(pub u32);

impl StatusCode {
    pub fn good() -> Self {
        Self(0)
    }
}
