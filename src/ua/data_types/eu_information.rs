//! Engineering units information.
//!
//! See also: <https://reference.opcfoundation.org/Core/Part8/v104/docs/5.6.3>

use crate::ua;

crate::data_type!(EUInformation);
crate::member_accessors!(EUInformation {
    #[from_inner]
    unitId: ua::UnitId,
    displayName: &ua::LocalizedText,
    description: &ua::LocalizedText,
    namespaceUri: &ua::String
});
