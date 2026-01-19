// [Part 6: 5.1.11 Null, Empty and Zero-Length Arrays](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.11)
// [Part 6: 5.2.2.16 Variant](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.16)
// [Part 6: 5.2.5 Arrays](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.5)
#[derive(Debug, Clone)]
pub struct Array<T>(pub Option<(Box<[T]>, Box<[i32]>)>);

impl<T> Array<T> {
    #[must_use]
    pub fn elements(&self) -> Option<&[T]> {
        self.0.as_ref().map(|array| array.0.as_ref())
    }

    #[must_use]
    pub fn dimensions(&self) -> Option<&[i32]> {
        self.0.as_ref().map(|array| array.1.as_ref())
    }

    #[must_use]
    pub fn iter(&self) -> Option<impl Iterator<Item = &T>> {
        self.0.as_ref().map(|array| array.0.iter())
    }

    #[must_use]
    pub fn into_vec(self) -> Option<Vec<T>> {
        self.0.map(|array| array.0.into_vec())
    }

    pub fn map<U, F>(self, f: F) -> Array<U>
    where
        F: FnMut(T) -> U,
    {
        todo!()
    }
}
