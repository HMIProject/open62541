// [Part 3: 8.17 Sbyte](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.17)
#[derive(Debug, Clone, Copy)]
pub struct SByte(pub i8);

impl SByte {
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}
