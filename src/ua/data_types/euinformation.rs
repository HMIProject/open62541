//! Engineering units information.
//!
//! See also: <https://reference.opcfoundation.org/Core/Part8/v104/docs/5.6.3>

use crate::{ua, DataType as _};

crate::data_type!(EUInformation);

impl EUInformation {
    /// The `unitId` field.
    #[must_use]
    pub const fn unit_id(&self) -> i32 {
        self.0.unitId
    }

    /// The `displayName` field.
    #[must_use]
    pub fn display_name(&self) -> &ua::LocalizedText {
        ua::LocalizedText::raw_ref(&self.0.displayName)
    }

    /// The `description` field.
    #[must_use]
    pub fn description(&self) -> &ua::LocalizedText {
        ua::LocalizedText::raw_ref(&self.0.description)
    }

    /// The UNECE code.
    ///
    /// Decoded from the [`unitId`](Self::unit_id).
    ///
    /// See also: <http://www.opcfoundation.org/UA/EngineeringUnits/UNECE/UNECE_to_OPCUA.csv>
    #[must_use]
    pub fn to_unece_code(&self) -> Option<String> {
        unece_code_from_unit_id(self.unit_id())
    }

    /// The abbreviated unit name.
    #[must_use]
    pub fn symbol(&self) -> &ua::String {
        // The Symbol field shall be copied to the EUInformation.displayName.
        // The localeId field of EUInformation.displayName shall be empty.
        self.display_name().text()
    }
}

#[expect(
    clippy::as_conversions,
    clippy::cast_sign_loss,
    reason = "conversion from i32 to u8"
)]
fn unece_code_from_unit_id(unit_id: i32) -> Option<String> {
    String::from_utf8(
        [
            ((unit_id & 0x00ff_0000) >> 16) as u8,
            ((unit_id & 0x0000_ff00) >> 8) as u8,
            (unit_id & 0x0000_00ff) as u8,
        ]
        .into_iter()
        .skip_while(|c| *c == 0x00)
        .collect(),
    )
    .ok()
}

#[cfg(test)]
mod tests {
    #[test]
    fn unece_code_from_unit_id() {
        assert_eq!(super::unece_code_from_unit_id(12592).unwrap(), "10"); // group
        assert_eq!(super::unece_code_from_unit_id(12851).unwrap(), "23"); // gram per cubic centimetre
        assert_eq!(super::unece_code_from_unit_id(17476).unwrap(), "DD"); // degree [unit of angle]
        assert_eq!(super::unece_code_from_unit_id(23130).unwrap(), "ZZ"); // mutually defined
        assert_eq!(super::unece_code_from_unit_id(4405297).unwrap(), "C81"); // radian
        assert_eq!(super::unece_code_from_unit_id(5910833).unwrap(), "Z11"); // hanging container
    }
}
