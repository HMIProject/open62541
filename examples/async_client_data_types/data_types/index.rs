// [Part 4: 7.18 Index](https://reference.opcfoundation.org/Core/Part4/v105/docs/7.18)
#[derive(Debug, Clone, Copy)]
pub struct Index(pub u32);

impl Index {
    pub fn zero() -> Self {
        Self(0)
    }
}
