use std::num::NonZeroU32;

use crate::data_types::Variant;

// [Part 3: 8.32 Structure](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.32)
// [Part 5: 12.2.12 Structure](https://reference.opcfoundation.org/Core/Part5/v105/docs/12.2.12)
// [Part 6: 5.1.7 Structures and Unions](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.7)
#[derive(Debug, Clone)]
pub enum Structure {
    Structure(Box<[Variant]>),
    StructureWithOptionalFields(Box<[Option<Variant>]>),
    StructureWithSubtypedValues,
    Union(Option<(NonZeroU32, Variant)>),
    UnionWithSubtypedValues,
}
