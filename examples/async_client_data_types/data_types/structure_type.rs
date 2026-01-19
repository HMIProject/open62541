// [Part 3: 8.49 StructureType](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.49)
// [Part 5: 12.2.5.3 StructureType](https://reference.opcfoundation.org/Core/Part5/v105/docs/12.2.5.3)
#[derive(Debug, Clone, Copy)]
pub enum StructureType {
    Structure,
    StructureWithOptionalFields,
    StructureWithSubtypedValues,
    Union,
    UnionWithSubtypedValues,
}
