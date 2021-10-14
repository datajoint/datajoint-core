use crate::util::generate_primitive_datajoint_enum;
use std::fmt::{Display, Formatter, Result};

generate_primitive_datajoint_enum! {
    #[doc="Error codes for library-related errors. All internal errors are"]
    #[doc="converted to one of these error codes so that the source of an error"]
    #[doc="can be easily identified by users of the C FFI.\n"]
    #[doc="At the moment, these error codes are not standardized. In other words,"]
    #[doc="the actual numeric value of the error may change at any time until"]
    #[doc="a numbering system is standardized."]
    #[repr(i32)]
    #[non_exhaustive]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
        InvalidEnumArgument,
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}
