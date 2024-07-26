use crate::{ua, DataType as _};

crate::data_type!(BrowsePathTarget);

impl BrowsePathTarget {
    #[must_use]
    pub fn target_id(&self) -> &ua::ExpandedNodeId {
        ua::ExpandedNodeId::raw_ref(&self.0.targetId)
    }

    /// Returns the index of the first unprocessed element in the [`ua::RelativePath`].
    ///
    /// This returns `None` if all elements were processed.
    ///
    /// # Panics
    ///
    /// The index must be in range of the compilation target's `usize` type. This is mostly a
    /// concern for 16-bit targets where index values larger than 65535 could not be represented.
    #[must_use]
    pub fn remaining_path_index(&self) -> Option<usize> {
        // The maximum index value indicates that all elements were processed.
        if self.0.remainingPathIndex == u32::MAX {
            return None;
        }
        // PANIC: It is not expected that the index is larger than 65535, so this should work even
        // on 16-bit compilation targets. On 32-bit or 64-bit targets, the limit is much higher.
        Some(
            self.0
                .remainingPathIndex
                .try_into()
                .expect("remaining path index should be in range of usize"),
        )
    }
}
