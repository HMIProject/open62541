// [Part 3: 8.36 UInt64](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.36)
#[derive(Debug, Clone, Copy)]
pub struct UInt64(pub u64);

impl UInt64 {
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}
