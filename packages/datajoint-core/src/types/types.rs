/// Generalized types supported by DataJoint.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

