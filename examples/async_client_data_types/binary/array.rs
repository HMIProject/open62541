use bytes::Bytes;

use crate::{
    binary::{BinaryReader, BinaryReaderWithContext},
    data_types::{Array, Int32},
};

// [Part 6: 5.2.5 Arrays](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.5)
impl<T> Array<T> {
    #[must_use]
    pub(crate) fn read_one_dimensional(data: &mut Bytes) -> Self
    where
        T: BinaryReader,
    {
        read_one_dimensional_array(data, |data| T::read(data))
    }

    #[must_use]
    pub(crate) fn read_multi_dimensional(data: &mut Bytes) -> Self
    where
        T: BinaryReader,
    {
        read_multi_dimensional_array(data, |data| T::read(data))
    }

    #[must_use]
    pub(crate) fn read_one_dimensional_with_context<C>(context: C, data: &mut Bytes) -> Self
    where
        T: BinaryReaderWithContext<C>,
        C: Copy,
    {
        read_one_dimensional_array(data, |data| T::read_with_context(context, data))
    }

    #[must_use]
    pub(crate) fn read_multi_dimensional_with_context<C>(context: C, data: &mut Bytes) -> Self
    where
        T: BinaryReaderWithContext<C>,
        C: Copy,
    {
        read_multi_dimensional_array(data, |data| T::read_with_context(context, data))
    }
}

// [Part 6: 5.2.5 Arrays](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.5)
fn read_one_dimensional_array<T, R>(data: &mut Bytes, mut read: R) -> Array<T>
where
    R: FnMut(&mut Bytes) -> T,
{
    let length = Int32::read(data).0;
    if length == -1 {
        return Array(None);
    }
    let dimensions = Box::new([length]);

    let number_of_values = usize::try_from(length).unwrap();
    let values = (0..number_of_values).map(|_| read(data)).collect();

    Array(Some((values, dimensions)))
}

// [Part 6: 5.2.5 Arrays](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.5)
fn read_multi_dimensional_array<T, R>(data: &mut Bytes, mut read: R) -> Array<T>
where
    R: FnMut(&mut Bytes) -> T,
{
    let dimensions = Array::<Int32>::read_one_dimensional(data);
    let Some(dimensions) = dimensions.into_vec() else {
        return Array(None);
    };
    let dimensions = dimensions
        .into_iter()
        .map(|dimension| dimension.0)
        .collect::<Box<[_]>>();

    let number_of_values = dimensions
        .iter()
        .map(|&dimension| usize::try_from(dimension.max(0)).unwrap())
        .product::<usize>();
    let values = (0..number_of_values).map(|_| read(data)).collect();

    Array(Some((values, dimensions)))
}
