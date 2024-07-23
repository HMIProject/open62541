/// Wrapper for specified attributes from [`open62541_sys`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecifiedAttributes(u32);

impl SpecifiedAttributes {
    /// Indicates if the `AccessLevel` Attribute is set.
    pub const ACCESSLEVEL: Self = Self(0);
    /// Indicates if the `ArrayDimensions` Attribute is set.
    pub const ARRAYDIMENSIONS: Self = Self(1);
    /// Indicates if the `ContainsNoLoops` Attribute is set.
    pub const CONTAINSNOLOOPS: Self = Self(3);
    /// Indicates if the `DataType` Attribute is set.
    pub const DATATYPE: Self = Self(4);
    /// Indicates if the `Description` Attribute is set.
    pub const DESCRIPTION: Self = Self(5);
    /// Indicates if the `DisplayName` Attribute is set.
    pub const DISPLAYNAME: Self = Self(6);
    /// Indicates if the `EventNotifier` Attribute is set.
    pub const EVENTNOTIFIER: Self = Self(7);
    /// Indicates if the `Executable` Attribute is set.
    pub const EXECUTABLE: Self = Self(8);
    /// Indicates if the `Historizing` Attribute is set.
    pub const HISTORIZING: Self = Self(9);
    /// Indicates if the `InverseName` Attribute is set.
    pub const INVERSENAME: Self = Self(10);
    /// Indicates if the `IsAbstract` Attribute is set.
    pub const ISABSTRACT: Self = Self(11);
    /// Indicates if the `MinimumSamplingInterval` Attribute is set.
    pub const MINIMUMSAMPLINGINTERVAL: Self = Self(12);
    /// Indicates if the `Symmetric` Attribute is set.
    pub const SYMMETRIC: Self = Self(15);
    /// Indicates if the `UserAccessLevel` Attribute is set.
    pub const USERACCESSLEVEL: Self = Self(16);
    /// Indicates if the `UserExecutable` Attribute is set.
    pub const USEREXECUTABLE: Self = Self(17);
    /// Indicates if the `UserWriteMask` Attribute is set.
    pub const USERWRITEMASK: Self = Self(18);
    /// Indicates if the `ValueRank` Attribute is set.
    pub const VALUERANK: Self = Self(19);
    /// Indicates if the `WriteMask` Attribute is set.
    pub const WRITEMASK: Self = Self(20);
    /// Indicates if the `Value` Attribute is set.
    pub const VALUE: Self = Self(21);

    pub(crate) const fn as_u32(&self) -> u32 {
        self.0
    }
}
