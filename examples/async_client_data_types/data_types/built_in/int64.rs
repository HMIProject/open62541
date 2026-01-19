// [Part 3: 8.27 Int64](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.37)
#[derive(Debug, Clone, Copy)]
pub struct Int64(pub i64);

impl Int64 {
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}
