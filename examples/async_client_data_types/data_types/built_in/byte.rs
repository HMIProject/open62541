// [Part 3: 8.9 Byte](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.9)
#[derive(Debug, Clone, Copy)]
pub struct Byte(pub u8);

impl Byte {
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}
