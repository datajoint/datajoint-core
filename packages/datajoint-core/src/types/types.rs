/// Generalized types supported by DataJoint.
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive)]
pub enum DataJointType {
    Unknown,
    TinyInt,
    TinyIntUnsigned,
    SmallInt,
    SmallIntUnsigned,
    MediumInt,
    MediumIntUnsigned,
    Int,
    IntUnsigned,
    Enum,
    Date,
    Time,
    DateTime,
    Timestamp,
    CharN,
    VarCharN,
    Float,
    Double,
    Decimal,
    TinyBlob,
    MediumBlob,
    Blob,
    LongBlob,
    Attach,
    FilepathStore,
}

