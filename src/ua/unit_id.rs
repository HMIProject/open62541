/// Wrapper for [`ua::EUInformation::unit_id`](crate::ua::EUInformation::unit_id).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnitId(i32);

impl UnitId {
    #[must_use]
    pub(crate) const fn new(unit_id: i32) -> Self {
        Self(unit_id)
    }

    /// The common UNECE code.
    ///
    /// Decoded from the wrapped integer value according to the
    /// [mapping defined by OPC UA](https://reference.opcfoundation.org/Core/Part8/v104/docs/5.6.3).
    ///
    /// The returned string is not guaranteed to represent a valid code.
    /// Returns `None` if the value could not be decoded.
    ///
    /// See also: <http://www.opcfoundation.org/UA/EngineeringUnits/UNECE/UNECE_to_OPCUA.csv>
    #[must_use]
    pub fn to_unece_code(&self) -> Option<String> {
        let Self(unit_id) = self;
        // TODO: More strict validation would require to inspect the official spec.
        String::from_utf8(
            unit_id
                .to_be_bytes()
                .iter()
                .copied()
                .skip_while(|c| *c == 0x00)
                .collect(),
        )
        .ok()
        .filter(|code| {
            // TODO: Add reference for minimum/maximum code length.
            code.len() >= 2 && code.len() <= 3
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ua::UnitId;

    #[test]
    fn unece_code_from_unit_id() {
        assert_eq!(UnitId::new(12_592).to_unece_code().unwrap(), "10"); // group
        assert_eq!(UnitId::new(12_851).to_unece_code().unwrap(), "23"); // gram per cubic centimetre
        assert_eq!(UnitId::new(17_476).to_unece_code().unwrap(), "DD"); // degree [unit of angle]
        assert_eq!(UnitId::new(23_130).to_unece_code().unwrap(), "ZZ"); // mutually defined
        assert_eq!(UnitId::new(4_405_297).to_unece_code().unwrap(), "C81"); // radian
        assert_eq!(UnitId::new(5_910_833).to_unece_code().unwrap(), "Z11"); // hanging container

        // Reject codes with invalid length.
        assert!(UnitId::new(0x0000_0000).to_unece_code().is_none());
        assert!(UnitId::new(0x3000_0000).to_unece_code().is_none());
        assert!(UnitId::new(0x0000_0030).to_unece_code().is_none());
        assert!(UnitId::new(0x3000_0030).to_unece_code().is_none());
        assert!(UnitId::new(0x0000_3030).to_unece_code().is_some());
        assert!(UnitId::new(0x3000_3030).to_unece_code().is_none());
        assert!(UnitId::new(0x0030_3030).to_unece_code().is_some());
        assert!(UnitId::new(0x3030_3030).to_unece_code().is_none());
    }
}
