crate::data_type!(Guid);

impl Guid {
    #[must_use]
    #[cfg(feature = "uuid")]
    pub fn from_uuid(uuid: uuid::Uuid) -> Self {
        use crate::DataType as _;
        let mut guid = Guid::init();
        let (data1, data2, data3, data4) = uuid.as_fields();
        guid.0.data1 = data1;
        guid.0.data2 = data2;
        guid.0.data3 = data3;
        guid.0.data4 = *data4;
        guid
    }

    #[must_use]
    #[cfg(feature = "uuid")]
    pub const fn to_uuid(&self) -> uuid::Uuid {
        uuid::Uuid::from_fields(self.0.data1, self.0.data2, self.0.data3, &self.0.data4)
    }
}

#[cfg(feature = "uuid")]
impl From<uuid::Uuid> for Guid {
    fn from(uuid: uuid::Uuid) -> Self {
        Self::from_uuid(uuid)
    }
}

#[cfg(all(feature = "serde", feature = "uuid"))]
impl serde::Serialize for Guid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_uuid().serialize(serializer)
    }
}
