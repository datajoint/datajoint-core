use std::fmt::{Display, Formatter, Result};

/// Error codes for library-related errors. All internal errors are
/// converted to one of these error codes so that the source of an error
/// can be easily identified by users of the C FFI.
///
/// At the moment, these error codes are not standardized. In other words,
/// the actual numeric value of the error may change at any time until
/// a numbering system is standardized.
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive)]
pub enum ErrorCode {
    Success = 0,

    // SQLx error codes.
    ConfigurationError,
    UnknownDatabaseError,
    IoError,
    TlsError,
    ProtocolError,
    RowNotFound,
    TypeNotFound,
    ColumnIndexOutOfBounds,
    ColumnNotFound,
    ColumnDecodeError,
    ValueDecodeError,
    PoolTimedOut,
    PoolClosed,
    WorkerCrashed,
    UnknownSqlxError,

    // DataJoint error codes.
    NotConnected,
    NoMoreRows,
    UnsupportedNativeType,
    WrongDatabaseType,
    UnexpectedNullValue,
    UnexpectedNoneType,

    // C FFI error codes.
    NullNotAllowed,
    BufferNotEnough,
    InvalidNativeType,
    InvalidCString,
    RowIndexOutOfBounds,
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

impl ErrorCode {
    /// Returns the standard message for the error code.
    pub fn standard_message(&self) -> &'static str {
        use ErrorCode::*;
        match self {
            Success => "success",
            ConfigurationError => "configuration error",
            UnknownDatabaseError => "unknown database error",
            IoError => "io error",
            TlsError => "tls error",
            ProtocolError => "protocol error",
            RowNotFound => "row not found",
            TypeNotFound => "type not found",
            ColumnIndexOutOfBounds => "column index out of bounds",
            ColumnNotFound => "column not found",
            ColumnDecodeError => "column decode error",
            ValueDecodeError => "value decode error",
            PoolTimedOut => "pool timed out",
            PoolClosed => "pool closed",
            WorkerCrashed => "worker crashed",
            UnknownSqlxError => "unknown sqlx error",

            NotConnected => "database not connected",
            NoMoreRows => "no more rows",
            UnsupportedNativeType => "unsupported native type",
            WrongDatabaseType => "wrong database type",
            UnexpectedNullValue => "unexpected null value encountered in decoding",
            UnexpectedNoneType => "unexpected none type encountered in encoding",

            NullNotAllowed => "null not allowed",
            BufferNotEnough => "buffer not enough",
            InvalidNativeType => "invalid native type",
            InvalidCString => "invalid c string",
            RowIndexOutOfBounds => "row index out of bounds",
        }
    }
}
