//! [5.2 OPC UA Binary](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2)

mod built_in;

use bytes::Bytes;

pub trait BinaryReader {
    fn read(data: &mut Bytes) -> Self;
}

pub trait StatefulBinaryReader {
    type Value;

    fn read_with_state(&mut self, data: &mut Bytes) -> Self::Value;
}

impl<T> StatefulBinaryReader for T
where
    T: BinaryReader,
{
    type Value = T;

    fn read_with_state(&mut self, data: &mut Bytes) -> Self::Value {
        T::read(data)
    }
}
