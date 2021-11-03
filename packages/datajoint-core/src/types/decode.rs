use crate::error::{DataJointError, Error, ErrorCode, SqlxError};
use crate::results::{TableColumnRef, TableRow};
use crate::types::DataJointType;
use sqlx::Row;
use std::fmt::{self, Display, Formatter};

/// Enum for a native type and its corresponding value that can be decoded
/// from a database or encoded into a query.
#[derive(Debug, Clone, PartialEq)]
pub enum NativeType {
    None,
    Bool(bool),
    Int8(i8),
    UInt8(u8),
    Int16(i16),
    UInt16(u16),
    Int32(i32),
    UInt32(u32),
    Int64(i64),
    UInt64(u64),
    String(String),
    Float32(f32),
    Float64(f64),
    Bytes(Vec<u8>),
}

impl Display for NativeType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use NativeType::*;
        match self {
            None => write!(f, "None"),
            Bool(val) => write!(f, "{}", val),
            Int8(int) => write!(f, "{}", int),
            UInt8(int) => write!(f, "{}", int),
            Int16(int) => write!(f, "{}", int),
            UInt16(int) => write!(f, "{}", int),
            Int32(int) => write!(f, "{}", int),
            UInt32(int) => write!(f, "{}", int),
            Int64(int) => write!(f, "{}", int),
            UInt64(int) => write!(f, "{}", int),
            String(string) => write!(f, "{}", string),
            Float32(float) => write!(f, "{}", float),
            Float64(float) => write!(f, "{}", float),
            Bytes(bytes) => match std::str::from_utf8(&bytes) {
                Err(_) => Err(std::fmt::Error),
                Ok(string) => write!(f, "{}", string),
            },
        }
    }
}

impl TableRow {
    fn postgres_unsupported_unsigned_error() -> Error {
        DataJointError::new_with_message(
            "postgres does not supported unsigned data types",
            ErrorCode::UnsupportedNativeType,
        )
    }

    /// Primary implementation of decoding a single column value in a single row.
    ///
    /// Handles null values by returning `None`.
    fn try_decode_impl(&self, column: TableColumnRef) -> Result<Option<NativeType>, Error> {
        use DataJointType::*;
        let index = column.ordinal();
        match column.type_name() {
            Unknown => Err(DataJointError::new_with_message(
                "unsupported column type",
                ErrorCode::ColumnDecodeError,
            )),
            // Need to look at https://docs.rs/sqlx/0.5.9/sqlx/types/index.html closer
            // for these types.
            FilepathStore => Err(DataJointError::new_with_message(
                "supported column type, but no decoder implemented",
                ErrorCode::ColumnDecodeError,
            )),
            Boolean => Ok(match self.try_get::<Option<bool>, usize>(index)? {
                None => None,
                Some(val) => Some(NativeType::Bool(val)),
            }),
            TinyInt => Ok(match self.try_get::<Option<i8>, usize>(index)? {
                None => None,
                Some(val) => Some(NativeType::Int8(val)),
            }),
            TinyIntUnsigned => match self {
                Self::MySql(row) => match row.try_get_unchecked::<Option<u8>, usize>(index) {
                    Err(err) => Err(SqlxError::new(err)),
                    Ok(None) => Ok(None),
                    Ok(Some(val)) => Ok(Some(NativeType::UInt8(val))),
                },
                Self::Postgres(_) => Err(TableRow::postgres_unsupported_unsigned_error()),
            },
            SmallInt => Ok(match self.try_get::<Option<i16>, usize>(index)? {
                None => None,
                Some(val) => Some(NativeType::Int16(val)),
            }),
            SmallIntUnsigned => match self {
                Self::MySql(row) => match row.try_get_unchecked::<Option<u16>, usize>(index) {
                    Err(err) => Err(SqlxError::new(err)),
                    Ok(None) => Ok(None),
                    Ok(Some(val)) => Ok(Some(NativeType::UInt16(val))),
                },
                Self::Postgres(_) => Err(TableRow::postgres_unsupported_unsigned_error()),
            },
            MediumInt | Int => Ok(match self.try_get::<Option<i32>, usize>(index)? {
                None => None,
                Some(val) => Some(NativeType::Int32(val)),
            }),
            MediumIntUnsigned | IntUnsigned => match self {
                Self::MySql(row) => match row.try_get_unchecked::<Option<u32>, usize>(index) {
                    Err(err) => Err(SqlxError::new(err)),
                    Ok(None) => Ok(None),
                    Ok(Some(val)) => Ok(Some(NativeType::UInt32(val))),
                },
                Self::Postgres(_) => Err(TableRow::postgres_unsupported_unsigned_error()),
            },
            BigInt => Ok(match self.try_get::<Option<i64>, usize>(index)? {
                None => None,
                Some(val) => Some(NativeType::Int64(val)),
            }),
            BigIntUnsigned => match self {
                Self::MySql(row) => match row.try_get_unchecked::<Option<u64>, usize>(index) {
                    Err(err) => Err(SqlxError::new(err)),
                    Ok(None) => Ok(None),
                    Ok(Some(val)) => Ok(Some(NativeType::UInt64(val))),
                },
                Self::Postgres(_) => Err(TableRow::postgres_unsupported_unsigned_error()),
            },
            Enum | CharN | VarCharN => Ok(match self.try_get::<Option<String>, usize>(index)? {
                None => None,
                Some(val) => Some(NativeType::String(val)),
            }),
            Date => Ok(match self.try_get::<Option<sqlx::types::chrono::NaiveDate>, usize>(index)? {
                None => None,
                Some(val) => Some(NativeType::String(val.to_string())),
            }),
            Time => Ok(match self.try_get::<Option<sqlx::types::chrono::NaiveTime>, usize>(index)? {
                None => None,
                Some(val) => Some(NativeType::String(val.to_string())),
            }),
            DateTime => Ok(match self.try_get::<Option<sqlx::types::chrono::NaiveDateTime>, usize>(index)? {
                None => None,
                Some(val) => Some(NativeType::String(val.to_string())),
            }),
            Timestamp => Ok(match self.try_get::<Option<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>, usize>(index)? {
                None => None,
                Some(val) => Some(NativeType::String(val.to_string())),
            }),
            Float => Ok(match self.try_get::<Option<f32>, usize>(index)? {
                None => None,
                Some(val) => Some(NativeType::Float32(val)),
            }),
            Double => Ok(match self.try_get::<Option<f64>, usize>(index)? {
                None => None,
                Some(val) => Some(NativeType::Float64(val)),
            }),
            TinyBlob | MediumBlob | Blob | LongBlob | Binary | Attach => Ok(match self.try_get::<Option<Vec<u8>>, usize>(index)? {
                None => None,
                Some(val) => Some(NativeType::Bytes(val)),
            }),
            Decimal => Ok(match self.try_get::<Option<sqlx::types::BigDecimal>, usize>(index)? {
                None => None,
                Some(val) => Some(NativeType::String(val.to_string())),
            }),
        }
    }

    /// Decodes the value at the given column depending on the type of the column,
    /// assuming it is not null.
    ///
    /// Panics on error.
    pub fn decode(&self, column: TableColumnRef) -> NativeType {
        self.try_decode(column).unwrap()
    }

    /// Decodes the value at the given column depending on the type of the column,
    /// assuming it is not null.
    ///
    /// Returns `ErrorCode::UnexpectedNullValue` on null values.
    pub fn try_decode(&self, column: TableColumnRef) -> Result<NativeType, Error> {
        match self.try_decode_impl(column)? {
            None => Err(DataJointError::new(ErrorCode::UnexpectedNullValue)),
            Some(val) => Ok(val),
        }
    }

    /// Decodes the value at the given column depending on the type of the column.
    /// Supports null values by returning `None`.
    ///
    /// Panics on error.
    pub fn decode_optional(&self, column: TableColumnRef) -> Option<NativeType> {
        self.try_decode_optional(column).unwrap()
    }

    /// Decodes the value at the given column depending on the type of the column.
    /// Supports null values by returning `None`.
    pub fn try_decode_optional(&self, column: TableColumnRef) -> Result<Option<NativeType>, Error> {
        self.try_decode_impl(column)
    }
}
