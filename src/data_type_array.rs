use std::mem::{self, MaybeUninit};

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

        Ok(())
    }

    /// Get [`ua::DataTypeArray`] of current array of data types.
    ///
    /// # Safety
    ///
    /// The returned object aliases the current set of items in `self`. It therefore must be dropped
    /// before making any other changes to `self`.
    #[must_use]
    pub(crate) unsafe fn to_data_type_array(&mut self) -> ua::DataTypeArray {
        // SAFETY: This transmute is allowed, because we go through `MaybeUninit` and `ua::DataType`
        // which both have `repr(transparent)`. We only access items that have been actually set.
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
