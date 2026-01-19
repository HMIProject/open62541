// [Part 3: 8.25 Int16](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.25)
#[derive(Debug, Clone, Copy)]
pub struct Int16(pub i16);

impl Int16 {
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}
