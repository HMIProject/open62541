// [Part 3: 8.34 UInt16](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.34)
#[derive(Debug, Clone, Copy)]
pub struct UInt16(pub u16);

impl UInt16 {
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}
