/// Native types that row values can be decoded to.
///
/// Should be parallel to datajoint_core::types::DecodeResult.
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NativeTypeEnum {
    None,
    Int8,
    UInt8,
    Int16,
    UInt16,
    Int32,
    UInt32,
    String,
    Float32,
    Float64,
    Bytes,
}
