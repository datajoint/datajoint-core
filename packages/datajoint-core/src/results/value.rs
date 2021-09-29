use sqlx::Value as SqlxValue;
use sqlx::ValueRef as SqlxValueRef;

/// Type trait for indicating if a type is safe to be decoded to.
///
/// Currently only implements how SQLx type checks return types.
pub trait ValueDecodable<'r>:
    sqlx::Decode<'r, sqlx::any::Any> + sqlx::Type<sqlx::any::Any>
{
}

impl<'r, T> ValueDecodable<'r> for T where
    T: sqlx::Decode<'r, sqlx::any::Any> + sqlx::Type<sqlx::any::Any>
{
}

/// A single value in a table row, which can be thought of as a single cell in a table.
///
/// Wraps `sqlx::any::AnyValue`.
pub struct Value {
    value: sqlx::any::AnyValue,
}

impl Value {
    /// Creates a new value around a SQLx value.
    pub fn new(value: sqlx::any::AnyValue) -> Self {
        Value { value: value }
    }

    /// Decodes the value into a given return type.
    ///
    /// Panics on error.
    pub fn decode<'r, T>(&'r self) -> T
    where
        T: ValueDecodable<'r>,
    {
        self.try_decode().unwrap()
    }

    /// Decodes the value into a given return type.
    pub fn try_decode<'r, T>(&'r self) -> Result<T, &str>
    where
        T: ValueDecodable<'r>,
    {
        // TODO(jnestelroad): Make sure this method does not panic.
        match self.value.try_decode() {
            Ok(value) => Ok(value),
            Err(_) => Err("error in decode"),
        }
    }
}

/// A reference to a single value in a table row.
///
/// Wraps `sqlx::any::AnyValueRef`.
pub struct ValueRef<'r> {
    value_ref: sqlx::any::AnyValueRef<'r>,
}

impl<'r> ValueRef<'r> {
    /// Creates a new value reference around a SQLx value reference.
    pub fn new(value_ref: sqlx::any::AnyValueRef<'r>) -> Self {
        ValueRef {
            value_ref: value_ref,
        }
    }

    /// Converts the reference to an owned version that can be decoded.
    pub fn to_owned(&self) -> Value {
        Value::new(self.value_ref.to_owned())
    }

    /// Checks if the value is null.
    pub fn is_null(&self) -> bool {
        self.value_ref.is_null()
    }
}
