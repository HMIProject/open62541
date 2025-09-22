//! Engineering units information.
//!
//! See also: <https://reference.opcfoundation.org/Core/Part8/v104/docs/5.6.3>

use crate::{DataType as _, ua};

crate::data_type!(EUInformation);

impl EUInformation {
    #[must_use]
    pub const fn unit_id(&self) -> ua::UnitId {
        ua::UnitId::new(self.0.unitId)
    }

    #[must_use]
    pub fn display_name(&self) -> &ua::LocalizedText {
        ua::LocalizedText::raw_ref(&self.0.displayName)
    }

    #[must_use]
    pub fn description(&self) -> &ua::LocalizedText {
        ua::LocalizedText::raw_ref(&self.0.description)
    }
}
