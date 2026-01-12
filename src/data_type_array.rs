use std::{
    fmt::Debug,
    marker::PhantomData,
    mem::{self, MaybeUninit},
    ptr,
};

use open62541_sys::{UA_DataType, UA_DataTypeArray};

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

    #[must_use]
    unsafe fn as_raw_parts(&mut self) -> (usize, *mut UA_DataType) {
        // SAFETY: This transmute is allowed, because `MaybeUninit` and `ua::DataType` both have the
        // attribute `#repr(transparent)`. We only access items that have been actually set.
        (self.len, unsafe { mem::transmute(self.items.as_mut_ptr()) })
    }

    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = &ua::DataType> {
        self.as_slice().into_iter()
    }

    /// Get [`ua::DataTypeArray`] of current array of data types.
    ///
    /// # Safety
    ///
    /// The returned object aliases the current set of items in `self`. It therefore must be dropped
    /// before making any other changes to `self`. In particular, `self` must outlive the result.
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

pub(crate) struct DataTypeArrayRef<'a>(UA_DataTypeArray, PhantomData<&'a ()>);

impl<'a> DataTypeArrayRef<'a> {
    pub(crate) fn new(arrays: Vec<&'a mut DataTypeArray>) -> Option<Self> {
        let mut result = None;

        // Iteratively prepend new values to the start of the resulting linked list. Resulting array
        // should have the same order, so we iterate over the incoming data type arrays in reverse.
        for array in arrays.into_iter().rev() {
            let (data_types_len, data_types) = unsafe { array.as_raw_parts() };

            let array = UA_DataTypeArray {
                next: ptr::null_mut(),
                typesSize: data_types_len,
                types: data_types,
                // We do not want the data type arrays inside the result to be cleaned up: the whole
                // idea is to alias them.
                cleanup: false,
            };

            result = Some(if let Some(array_chain) = result {
                UA_DataTypeArray {
                    // Leak any referenced chained data type arrays onto the heap. Drop will have to
                    // recapture them. This is the nature of a linked list.
                    next: Box::leak(Box::new(array_chain)),
                    ..array
                }
            } else {
                array
            });
        }

        result.map(|array_chain| Self(array_chain, PhantomData))
    }

    /// Get [`ua::DataTypeArray`] of current array of data types.
    ///
    /// # Safety
    ///
    /// The returned object aliases the current set of items in `self`. It therefore must be dropped
    /// before making any other changes to `self`. In particular, `self` must outlive the result.
    #[must_use]
    pub(crate) unsafe fn to_data_type_array(&mut self) -> ua::DataTypeArray {
        // TODO: Use constructor method instead.
        unsafe { mem::transmute(ptr::read(&raw const self.0)) }
    }
}

impl Drop for DataTypeArrayRef<'_> {
    fn drop(&mut self) {
        let mut next = self.0.next;

        while !next.is_null() {
            let array = unsafe { Box::from_raw(next) };
            next = array.next;
            // Heap allocation is cleaned up here when `array` goes out of scope.
        }
    }
}
