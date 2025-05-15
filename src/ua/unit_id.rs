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
    /// Returns an ASCII alphanumeric string with a minimum length of 1 and
    /// a maximum length of 3. All other strings are discarded as invalid.
    /// `unit_id`s where the most significant byte is not `0x00` are rejected
    /// as invalid.
    ///
    /// Returns `None` if the value could not be decoded or is invalid.
    ///
    /// The returned string is not guaranteed to represent a valid _Common Code_
    /// as defined by the [UNECE documents] or the [OPC UA specification].
    ///
    /// [UNECE documents]: <https://unece.org/trade/documents/session-documents/revision-6>
    /// [OPC UA specification]: <http://www.opcfoundation.org/UA/EngineeringUnits/UNECE/UNECE_to_OPCUA.csv>
    #[must_use]
    pub fn to_unece_code(&self) -> Option<String> {
        let Self(unit_id) = self;
        let ascii_chars = unit_id
            .to_be_bytes()
            .iter()
            .copied()
            .skip_while(|c| *c == 0x00)
            .map(|c| {
                // TODO: Are lowercase ASCII characters allowed? Probably not.
                c.is_ascii_alphanumeric().then_some(c)
            })
            .collect::<Option<Vec<_>>>()?;
        // TODO: Restrict minimum length to 2?
        if !(1..=3).contains(&ascii_chars.len()) {
            return None;
        }
        let code = String::from_utf8(ascii_chars);
        debug_assert!(code.is_ok(), "never fails for ASCII character codes");
        code.ok()
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

        // Allow only codes with expected length.
        assert!(UnitId::new(0x0000_0000).to_unece_code().is_none());
        assert!(UnitId::new(0x3000_0000).to_unece_code().is_none());
        assert!(
            UnitId::new(0x0000_0030).to_unece_code().is_some(),
            "valid code of 1 ASCII character"
        );
        assert!(UnitId::new(0x3000_0030).to_unece_code().is_none());
        assert!(
            UnitId::new(0x0000_3030).to_unece_code().is_some(),
            "valid code of 2 ASCII characters"
        );
        assert!(UnitId::new(0x3000_3030).to_unece_code().is_none());
        assert!(
            UnitId::new(0x0030_3030).to_unece_code().is_some(),
            "valid code of 3 ASCII characters"
        );
        assert!(UnitId::new(0x3030_3030).to_unece_code().is_none());
    }
}
