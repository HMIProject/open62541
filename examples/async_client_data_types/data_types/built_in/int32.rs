// [Part 3: 8.26 Int32](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.32)
#[derive(Debug, Clone, Copy)]
pub struct Int32(pub i32);

impl Int32 {
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}
