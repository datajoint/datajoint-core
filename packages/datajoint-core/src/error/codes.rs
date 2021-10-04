use std::fmt::{Display, Formatter, Result};

/// Error codes for library-related errors. All internal errors are
/// converted to one of these error codes so that the source of an error
/// can be easily identified by users of the C FFI.
///
/// At the moment, these error codes are not standardized. In other words,
/// the actual numeric value of the error may change at any time until
/// a numbering system is standardized.
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    Success = 0,
    Unknown,

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

    NotConnected,
    NoMoreRows,
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}
