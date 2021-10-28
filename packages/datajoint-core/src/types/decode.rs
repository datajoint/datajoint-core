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

    /// Decodes the value at the given column depending on the type of the column.
    ///
    /// Panics on error.
    pub fn decode(&self, column: TableColumnRef) -> NativeType {
        self.try_decode(column).unwrap()
    }

    /// Decodes the value at the given column depending on the type of the column.
    pub fn try_decode(&self, column: TableColumnRef) -> Result<NativeType, Error> {
        use DataJointType::*;
        let index = column.ordinal();
        match column.type_name() {
            Unknown => Err(DataJointError::new_with_message(
                "unsupported column type",
                ErrorCode::ColumnDecodeError,
            )),
            // Need to look at https://docs.rs/sqlx/0.5.9/sqlx/types/index.html closer
            // for these types.
            Decimal | Attach | FilepathStore => Err(DataJointError::new_with_message(
                "supported column type, but no decoder implemented",
                ErrorCode::ColumnDecodeError,
            )),
            Boolean => Ok(NativeType::Bool(self.try_get::<bool, usize>(index)?)),
            TinyInt => Ok(NativeType::Int8(self.try_get::<i8, usize>(index)?)),
            TinyIntUnsigned => match self {
                Self::MySql(row) => match row.try_get_unchecked::<u8, usize>(index) {
                    Err(err) => Err(SqlxError::new(err)),
                    Ok(val) => Ok(NativeType::UInt8(val)),
                },
                Self::Postgres(_) => Err(TableRow::postgres_unsupported_unsigned_error()),
            },
            SmallInt => Ok(NativeType::Int16(self.try_get::<i16, usize>(index)?)),
            SmallIntUnsigned => match self {
                Self::MySql(row) => match row.try_get_unchecked::<u16, usize>(index) {
                    Err(err) => Err(SqlxError::new(err)),
                    Ok(val) => Ok(NativeType::UInt16(val)),
                },
                Self::Postgres(_) => Err(TableRow::postgres_unsupported_unsigned_error()),
            },
            MediumInt | Int => Ok(NativeType::Int32(self.try_get::<i32, usize>(index)?)),
            MediumIntUnsigned | IntUnsigned => match self {
                Self::MySql(row) => match row.try_get_unchecked::<u32, usize>(index) {
                    Err(err) => Err(SqlxError::new(err)),
                    Ok(val) => Ok(NativeType::UInt32(val)),
                },
                Self::Postgres(_) => Err(TableRow::postgres_unsupported_unsigned_error()),
            },
            BigInt => Ok(NativeType::Int64(self.try_get::<i64, usize>(index)?)),
            BigIntUnsigned => match self {
                Self::MySql(row) => match row.try_get_unchecked::<u64, usize>(index) {
                    Err(err) => Err(SqlxError::new(err)),
                    Ok(val) => Ok(NativeType::UInt64(val)),
                },
                Self::Postgres(_) => Err(TableRow::postgres_unsupported_unsigned_error()),
            },
            Enum | CharN | VarCharN => {
                Ok(NativeType::String(self.try_get::<String, usize>(index)?))
            }
            Date => Ok(NativeType::String(
                self.try_get::<sqlx::types::chrono::NaiveDate, usize>(index)?
                    .to_string(),
            )),
            Time => Ok(NativeType::String(
                self.try_get::<sqlx::types::chrono::NaiveTime, usize>(index)?
                    .to_string(),
            )),
            DateTime => Ok(NativeType::String(
                self.try_get::<sqlx::types::chrono::NaiveDateTime, usize>(index)?
                    .to_string(),
            )),
            Timestamp => Ok(NativeType::String(
                self.try_get::<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>, usize>(
                    index,
                )?
                .to_string(),
            )),
            Float => Ok(NativeType::Float32(self.try_get::<f32, usize>(index)?)),
            Double => Ok(NativeType::Float64(self.try_get::<f64, usize>(index)?)),
            TinyBlob | MediumBlob | Blob | LongBlob => {
                Ok(NativeType::Bytes(self.try_get::<Vec<u8>, usize>(index)?))
            }
        }
    }
}
