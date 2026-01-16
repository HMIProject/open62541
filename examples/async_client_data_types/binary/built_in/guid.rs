use bytes::Bytes;

use crate::{
    binary::BinaryReader,
    data_types::{Byte, Guid, UInt16, UInt32},
};

// [Part 6: 5.2.2.6 Guid](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.6)
impl BinaryReader for Guid {
    fn read(data: &mut Bytes) -> Self {
        let a = UInt32::read(data);
        let b = UInt16::read(data);
        let c = UInt16::read(data);
        let d = [
            Byte::read(data),
            Byte::read(data),
            Byte::read(data),
            Byte::read(data),
            Byte::read(data),
            Byte::read(data),
            Byte::read(data),
            Byte::read(data),
        ];
        Self(a.0, b.0, c.0, d.map(|byte| byte.0))
    }
}
