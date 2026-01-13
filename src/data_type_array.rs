use std::{
    fmt::Debug,
    mem::{self, MaybeUninit},
    num::NonZeroUsize,
    pin::Pin,
    ptr, slice,
};

use open62541_sys::{UA_DataType, UA_DataTypeArray};

use crate::ua;

/// Safer variant of [`ua::DataTypeArray`].
///
/// This tracks the array's capacity, allowing in-place appending of new items without creating lots
/// of singleton arrays in the resulting linked list of arrays.
pub(crate) struct DataTypeArray {
    // List of arrays that will form the linked list in `ua::DataTypeArray`. These are pinned for us
    // to be able to return `ua::DataTypeArray` instances that can live across resizes of the vector
    // `arrays` itself.
    arrays: Vec<Pin<Box<UA_DataTypeArray>>>,

    // Capacity of final item in `self.arrays`. The other items will be assumed to be fully occupied
    // (i.e., their capacity matches their length).
    last_capacity: NonZeroUsize,
}

unsafe impl Send for DataTypeArray {}

unsafe impl Sync for DataTypeArray {}

const INITIAL_ARRAY_CAPACITY: NonZeroUsize = NonZeroUsize::new(4).expect("positive value");

impl DataTypeArray {
    #[must_use]
    pub(crate) fn new() -> Self {
        let last_capacity = INITIAL_ARRAY_CAPACITY;

        Self {
            arrays: vec![uninit_array(last_capacity, None)],
            last_capacity,
        }
    }

    pub(crate) fn push(&mut self, data_type: ua::DataType) {
        // Insert data type into free position in last array in list. We repeat this once, in case a
        // full array is found in the first try and we need to extend the list of arrays first.
        for _ in 0..2 {
            // PANIC: We always create `DataTypeArray` with a non-empty list of arrays.
            let last_array = self.arrays.last_mut().expect("empty list of arrays");
            // SAFETY: We reconstitute the slice as it was intially created.
            let types = unsafe {
                slice::from_raw_parts_mut(
                    last_array.types.cast::<MaybeUninit<UA_DataType>>(),
                    self.last_capacity.get(),
                )
            };

            if let Some(last_type) = types.get_mut(last_array.typesSize) {
                last_type.write(data_type.into_raw());
                last_array.typesSize += 1;

                return;
            }

            // Last array in list is full. Push new array to list. Double capacity of each new array
            // for amortized constant time of this operation.
            let factor = NonZeroUsize::new(2).expect("positive factor");
            let next_capacity = self.last_capacity.saturating_mul(factor);

            let array = uninit_array(next_capacity, Some(last_array.as_mut()));
            self.arrays.push(array);
            self.last_capacity = next_capacity;
        }

        // PANIC: This never happens: we just added a new array and it has non-zero capacity.
        unreachable!("newly added array is full");
    }

    #[expect(dead_code, reason = "unused for now")]
    #[must_use]
    pub(crate) fn len(&self) -> usize {
        self.arrays.iter().map(|array| array.typesSize).sum()
    }

    #[expect(dead_code, reason = "unused for now")]
    #[must_use]
    pub(crate) fn is_empty(&self) -> bool {
        // PANIC: We always create `DataTypeArray` with a non-empty list of arrays.
        let last_array = self.arrays.last().expect("empty list of arrays");

        // It is sufficient to check the size of any array in the list, because only the last one is
        // possibly empty: all others are always assumed to be at full (non-zero) capacity.
        last_array.typesSize != 0
    }

    #[must_use]
    pub(crate) fn contains(&self, type_id: &ua::NodeId) -> bool {
        self.iter().any(|r#type| r#type.type_id() == type_id)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &ua::DataType> {
        self.arrays
            .iter()
            .map(|array| {
                // SAFETY: We only access the initialized parts of the array.
                unsafe { slice::from_raw_parts(array.types, array.typesSize) }
            })
            .flat_map(|types| types.iter().map(ua::DataType::raw_ref))
    }

    /// Get [`ua::DataTypeArray`] of current array of data types.
    #[must_use]
    pub(crate) fn as_data_type_array(&self) -> Pin<&ua::DataTypeArray> {
        // PANIC: We always create `DataTypeArray` with a non-empty list of arrays.
        let first_array = self.arrays.first().expect("non-empty list of arrays");

        unsafe {
            // SAFETY: Transmutation is allowed for `#[repr(transparent)]`.
            mem::transmute::<Pin<&UA_DataTypeArray>, Pin<&ua::DataTypeArray>>(first_array.as_ref())
        }
    }
}

impl Drop for DataTypeArray {
    fn drop(&mut self) {
        // PANIC: We always create `DataTypeArray` with a non-empty list of arrays.
        let (last_array, arrays) = self
            .arrays
            .split_last_mut()
            .expect("non-empty list of arrays");

        // Drop arrays one after the other.
        for array in arrays {
            // PANIC: All but the last array in the list are filled to capacity. Capacity of all the
            // arrays in the list is non-zero.
            let capacity = NonZeroUsize::new(array.typesSize).expect("non-empty array");
            unsafe { drop_array(array, capacity) };
        }

        // Drop last array (which may be partially or entirely empty).
        unsafe { drop_array(last_array, self.last_capacity) };
    }
}

impl Debug for DataTypeArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Add useful implementation.
        f.debug_tuple("DataTypeArray").finish_non_exhaustive()
    }
}

fn uninit_array(
    capacity: NonZeroUsize,
    next: Option<Pin<&mut UA_DataTypeArray>>,
) -> Pin<Box<UA_DataTypeArray>> {
    let mut types = Vec::with_capacity(capacity.get());

    // Construct vector (to be turned into boxed slice) manually, because `UA_DataType` (and wrapper
    // `ua::DataType`) does not implement `Clone`.
    while types.len() < types.capacity() {
        types.push(MaybeUninit::uninit());
    }

    // PANIC: Uphold invariant for reclaiming the owned memory later.
    debug_assert_eq!(types.len(), types.capacity(), "unexpected size");
    let types: &mut [MaybeUninit<UA_DataType>] = types.leak::<'static>();

    // Casting from `MaybeUninit<UA_DataType>` to `UA_DataType` is allowed, because `MaybeUninit` is
    // `#[repr(transparent)]`. We make sure that we only access items at this pointer within the set
    // capacity. When reassembling the owned memory (for dropping), we apply the correct capacity.
    let types = types.as_mut_ptr().cast::<UA_DataType>();

    Box::pin(UA_DataTypeArray {
        next: next.map_or(ptr::null_mut(), |mut next| &raw mut *next),
        typesSize: 0,
        types,
        // This flag only ever gets used by `UA_cleanupDataTypeWithCustom()` which we do not use. We
        // always clean up in our own `Drop` implementation.
        cleanup: false,
    })
}

unsafe fn drop_array(array: &mut Pin<Box<UA_DataTypeArray>>, capacity: NonZeroUsize) {
    // SAFETY: We reconstitute the owned memory as it was intially created.
    let mut types = unsafe {
        Vec::from_raw_parts(
            array.types.cast::<MaybeUninit<UA_DataType>>(),
            array.typesSize,
            capacity.get(),
        )
    };

    for r#type in &mut types {
        // SAFETY: These are the types that have actually been set.
        unsafe { r#type.assume_init_drop() };
    }
}
