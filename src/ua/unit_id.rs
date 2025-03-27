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
    /// Returns `None` if the value could not be decoded. The returned string
    /// is not guaranteed to represent a valid code.
    ///
    /// See also: <http://www.opcfoundation.org/UA/EngineeringUnits/UNECE/UNECE_to_OPCUA.csv>
    #[must_use]
    #[allow(clippy::missing_panics_doc)] // Bitmasks restrict values.
    pub fn to_unece_code(&self) -> Option<String> {
        let Self(unit_id) = self;
        // TODO: More strict validation would require to inspect the official spec.
        String::from_utf8(
            [
                u8::try_from((unit_id & 0x00ff_0000) >> 16).expect("always in range"),
                u8::try_from((unit_id & 0x0000_ff00) >> 8).expect("always in range"),
                u8::try_from(unit_id & 0x0000_00ff).expect("always in range"),
            ]
            .into_iter()
            .skip_while(|c| *c == 0x00)
            .collect(),
        )
        .ok()
    }
}

#[cfg(test)]
mod tests {
    use crate::ua::UnitId;

    #[test]
    fn unece_code_from_unit_id() {
        assert_eq!(UnitId::new(12592).to_unece_code().unwrap(), "10"); // group
        assert_eq!(UnitId::new(12851).to_unece_code().unwrap(), "23"); // gram per cubic centimetre
        assert_eq!(UnitId::new(17476).to_unece_code().unwrap(), "DD"); // degree [unit of angle]
        assert_eq!(UnitId::new(23130).to_unece_code().unwrap(), "ZZ"); // mutually defined
        assert_eq!(UnitId::new(4405297).to_unece_code().unwrap(), "C81"); // radian
        assert_eq!(UnitId::new(5910833).to_unece_code().unwrap(), "Z11"); // hanging container
    }
}
