pub struct SpecifiedAttributes;

#[allow(dead_code)] // Not all values are used.
#[allow(clippy::doc_markdown)] // Quoted from spec.
impl SpecifiedAttributes {
    /// Indicates if the AccessLevel Attribute is set.
    pub const ACCESSLEVEL: u32 = 0;
    /// Indicates if the ArrayDimensions Attribute is set.
    pub const ARRAYDIMENSIONS: u32 = 1;
    /// Indicates if the ContainsNoLoops Attribute is set.
    pub const CONTAINSNOLOOPS: u32 = 3;
    /// Indicates if the DataType Attribute is set.
    pub const DATATYPE: u32 = 4;
    /// Indicates if the Description Attribute is set.
    pub const DESCRIPTION: u32 = 5;
    /// Indicates if the DisplayName Attribute is set.
    pub const DISPLAYNAME: u32 = 6;
    /// Indicates if the EventNotifier Attribute is set.
    pub const EVENTNOTIFIER: u32 = 7;
    /// Indicates if the Executable Attribute is set.
    pub const EXECUTABLE: u32 = 8;
    /// Indicates if the Historizing Attribute is set.
    pub const HISTORIZING: u32 = 9;
    /// Indicates if the InverseName Attribute is set.
    pub const INVERSENAME: u32 = 10;
    /// Indicates if the IsAbstract Attribute is set.
    pub const ISABSTRACT: u32 = 11;
    /// Indicates if the MinimumSamplingInterval Attribute is set.
    pub const MINIMUMSAMPLINGINTERVAL: u32 = 12;
    /// Indicates if the Symmetric Attribute is set.
    pub const SYMMETRIC: u32 = 15;
    /// Indicates if the UserAccessLevel Attribute is set.
    pub const USERACCESSLEVEL: u32 = 16;
    /// Indicates if the UserExecutable Attribute is set.
    pub const USEREXECUTABLE: u32 = 17;
    /// Indicates if the UserWriteMask Attribute is set.
    pub const USERWRITEMASK: u32 = 18;
    /// Indicates if the ValueRank Attribute is set.
    pub const VALUERANK: u32 = 19;
    /// Indicates if the WriteMask Attribute is set.
    pub const WRITEMASK: u32 = 20;
    /// Indicates if the Value Attribute is set.
    pub const VALUE: u32 = 21;
}
