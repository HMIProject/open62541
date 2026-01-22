use open62541_sys::UA_Guid;

crate::data_type!(Guid);

impl Guid {
    #[must_use]
    pub fn new(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        Self(UA_Guid {
            data1,
            data2,
            data3,
            data4,
        })
    }

    #[must_use]
    pub const fn data1(&self) -> u32 {
        self.0.data1
    }

    #[must_use]
    pub const fn data2(&self) -> u16 {
        self.0.data2
    }

    #[must_use]
    pub const fn data3(&self) -> u16 {
        self.0.data3
    }

    #[must_use]
    pub const fn data4(&self) -> [u8; 8] {
        self.0.data4
    }
}

#[cfg(feature = "uuid")]
impl Guid {
    /// Creates a [`Guid`] from a [`Uuid`](uuid::Uuid).
    #[must_use]
    #[cfg(feature = "uuid")]
    pub fn from_uuid(uuid: uuid::Uuid) -> Self {
        let (data1, data2, data3, data4) = uuid.as_fields();

        Self::new(data1, data2, data3, *data4)
    }

    /// Converts the [`Guid`] into a [`Uuid`](uuid::Uuid).
    ///
    /// The bitwise conversion might not result in a valid v1/v2/v3/v4/v5/v6/v7/v8 UUID.
    #[must_use]
    pub const fn to_uuid(&self) -> uuid::Uuid {
        uuid::Uuid::from_fields(self.data1(), self.data2(), self.data3(), &self.data4())
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
