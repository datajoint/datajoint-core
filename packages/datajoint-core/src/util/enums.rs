use crate::connection::DatabaseType;
use crate::error::ErrorCode;
use crate::types::DataJointType;
use num_traits::FromPrimitive;

/// Trait for converting integer enum types to the Rust enum type.
///
/// Wraps the `num::traits::FromPrimitive` trait so it does not need
/// to be imported by library users.
pub trait IntegerEnum<I>
where
    I: num_traits::int::PrimInt,
    Self: Sized,
{
    /// Attempts to convert the integer value to the enum type.
    ///
    /// Returns `None` if integer is invalid.
    fn from_int(val: I) -> Option<Self>;
}

impl IntegerEnum<i32> for DatabaseType {
    fn from_int(val: i32) -> Option<Self> {
        FromPrimitive::from_i32(val)
    }
}

impl IntegerEnum<i32> for ErrorCode {
    fn from_int(val: i32) -> Option<Self> {
        FromPrimitive::from_i32(val)
    }
}

impl IntegerEnum<i32> for DataJointType {
    fn from_int(val: i32) -> Option<Self> {
        FromPrimitive::from_i32(val)
    }
}
