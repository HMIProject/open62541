// [Part 3: 8.16 Guid](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.16)
pub struct Guid(pub u32, pub u16, pub u16, pub [u8; 8]);

impl Guid {
    pub fn is_zero(&self) -> bool {
        let Self(a, b, c, d) = self;

        *a == 0 && *b == 0 && *c == 0 && *d == [0, 0, 0, 0, 0, 0, 0, 0]
    }
}
