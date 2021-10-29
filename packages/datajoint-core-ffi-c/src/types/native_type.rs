/// Native types that can be decoded from a database or encoded to a query,
/// possibly for a placeholder argument.
///
/// Should be parallel to `datajoint_core::types::NativeType`.
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NativeTypeEnum {
    None,
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
