use crate::error::{DataJointError, Error, ErrorCode};
use crate::results::{TableColumnRef, TableRow};
use crate::types::DataJointType;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum DecodeResult {
    Int8(i8),
    UInt8(u8),
    Int16(i16),
    UInt16(u16),
    Int32(i32),
    UInt32(u32),
    String(String),
    Float32(f32),
    Float64(f64),
    Bytes(Vec<u8>),
}

impl Display for DecodeResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use DecodeResult::*;
        match self {
            Int8(int) => write!(f, "{}", int),
            UInt8(int) => write!(f, "{}", int),
            Int16(int) => write!(f, "{}", int),
            UInt16(int) => write!(f, "{}", int),
            Int32(int) => write!(f, "{}", int),
            UInt32(int) => write!(f, "{}", int),
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
    /// Decodes the value at the given column depending on the type of the column.
    ///
    /// Panics on error.
    pub fn decode(&self, column: TableColumnRef) -> DecodeResult {
        self.try_decode(column).unwrap()
    }

    /// Decodes the value at the given column depending on the type of the column.
    pub fn try_decode(&self, column: TableColumnRef) -> Result<DecodeResult, Error> {
        use DataJointType::*;
        let index = column.ordinal();
        match column.type_name() {
            Unknown => Err(DataJointError::new(
                "unsupported column type",
                ErrorCode::ColumnDecodeError,
            )),
            // Need to look at https://docs.rs/sqlx/0.5.9/sqlx/types/index.html closer
            // for these types.
            Decimal | Attach | FilepathStore => Err(DataJointError::new(
                "supported column type, but no decoder implemented",
                ErrorCode::ColumnDecodeError,
            )),
            TinyInt => Ok(DecodeResult::Int8(self.try_get::<i32, usize>(index)? as i8)),
            TinyIntUnsigned => Ok(DecodeResult::UInt8(self.try_get::<i32, usize>(index)? as u8)),
            SmallInt => Ok(DecodeResult::Int16(
                self.try_get::<i32, usize>(index)? as i16
            )),
            SmallIntUnsigned => Ok(DecodeResult::UInt16(
                self.try_get::<i32, usize>(index)? as u16
            )),
            MediumInt | Int => Ok(DecodeResult::Int32(self.try_get::<i32, usize>(index)?)),
            MediumIntUnsigned | IntUnsigned => Ok(DecodeResult::UInt32(
                self.try_get::<i32, usize>(index)? as u32,
            )),
            Enum | CharN | VarCharN => {
                Ok(DecodeResult::String(self.try_get::<String, usize>(index)?))
            }
            Date => Ok(DecodeResult::String(
                self.try_get::<sqlx::types::chrono::NaiveDate, usize>(index)?
                    .to_string(),
            )),
            Time => Ok(DecodeResult::String(
                self.try_get::<sqlx::types::chrono::NaiveTime, usize>(index)?
                    .to_string(),
            )),
            DateTime => Ok(DecodeResult::String(
                self.try_get::<sqlx::types::chrono::NaiveDateTime, usize>(index)?
                    .to_string(),
            )),
            Timestamp => Ok(DecodeResult::String(
                self.try_get::<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>, usize>(
                    index,
                )?
                    .to_string(),
            )),
            Float => Ok(DecodeResult::Float32(self.try_get::<f32, usize>(index)?)),
            Double => Ok(DecodeResult::Float64(self.try_get::<f64, usize>(index)?)),
            TinyBlob | MediumBlob | Blob | LongBlob => {
                Ok(DecodeResult::Bytes(self.try_get::<Vec<u8>, usize>(index)?))
            }
        }
    }
}