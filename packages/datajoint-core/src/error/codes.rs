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

    // C FFI error codes.
    NullNotAllowed,
    BufferNotEnough,
    InvalidNativeType,
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}
