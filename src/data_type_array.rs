use std::{
    fmt::Debug,
    mem::{self, MaybeUninit},
};

use open62541_sys::UA_DataType;

use crate::{Error, Result, ua};

/// Safer variant of [`ua::DataTypeArray`].
pub(crate) struct DataTypeArray {
    items: Box<[MaybeUninit<ua::DataType>]>,
    len: usize,
}

impl DataTypeArray {
    pub(crate) fn new(capacity: usize) -> Self {
        let mut items = Vec::with_capacity(capacity);

        // Construct vector (to be turned into boxed slice) manually, because `UA_DataType` (and our
        // wrapper `ua::DataType`) does not implement `Clone`.
        for _ in 0..capacity {
            items.push(MaybeUninit::uninit());
        }

        Self {
            items: items.into_boxed_slice(),
            len: 0,
        }
    }

    pub(crate) fn push(&mut self, data_type: ua::DataType) -> Result<()> {
        let Some(item) = self.items.get_mut(self.len) else {
            return Err(Error::internal("maximum length reached"));
        };

        item.write(data_type);
        self.len += 1;

        Ok(())
    }

    #[must_use]
    pub(crate) fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub(crate) fn is_empty(&self) -> bool {
        self.len != 0
    }

    #[must_use]
    pub(crate) fn as_slice(&self) -> &[ua::DataType] {
        // SAFETY: This transmute is allowed, because `MaybeUninit` has `#repr(transparent)` set. We
        // only access items that have been actually set.
        unsafe { mem::transmute(&self.items[0..self.len]) }
    }

    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = &ua::DataType> {
        self.as_slice().into_iter()
    }

    /// Get [`ua::DataTypeArray`] of current array of data types.
    ///
    /// # Safety
    ///
    /// The returned object aliases the current set of items in `self`. It therefore must be dropped
    /// before making any other changes to `self`.
    #[must_use]
    pub(crate) unsafe fn to_data_type_array(&mut self) -> ua::DataTypeArray {
        // SAFETY: This transmute is allowed, because `MaybeUninit` and `ua::DataType` both have the
        // attribute `#repr(transparent)`. We only access items that have been actually set.
        let data_types = unsafe { mem::transmute(&mut self.items[0..self.len]) };

        unsafe { ua::DataTypeArray::new(data_types) }
    }
}

impl Drop for DataTypeArray {
    fn drop(&mut self) {
        for item in &mut self.items[0..self.len] {
            // SAFETY: These are exactly the items that have been actually set. Since they would not
            // be dropped automatically within `MaybeUninit`, we do it manually here.
            unsafe { item.assume_init_drop() };
        }
    }
}

impl Debug for DataTypeArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Add useful implementation.
        f.debug_tuple("DataTypeArray").finish_non_exhaustive()
    }
}
