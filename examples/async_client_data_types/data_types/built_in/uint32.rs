// [Part 3: 8.35 UInt32](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.35)
#[derive(Debug, Clone, Copy)]
pub struct UInt32(pub u32);

impl UInt32 {
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}
