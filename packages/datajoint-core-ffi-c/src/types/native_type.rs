/// Native types that can be decoded from a database or encoded to a query,
/// possibly for a placeholder argument.
///
/// Should be parallel to `datajoint_core::types::NativeType`, aside from the
/// additional variant to represent null.
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NativeTypeEnum {
    /// Represents the complete absence of any value.
    None,

    /// Represents a null value.
    Null,

    Bool,
    Int8,
    UInt8,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
    String,
    Float32,
    Float64,
    Bytes,
}
