use crate::error::ErrorCode;
use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter, Result};

/// Error type that can be returned from library functions.
pub type Error = Box<dyn LibraryError>;

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.message())
    }
}

/// Trait for all errors in library functions, regardless of their origin.
pub trait LibraryError {
    /// Error message.
    fn message(&self) -> Cow<'_, str>;

    /// DataJoint error code.
    fn code(&self) -> ErrorCode;

    /// Associated SQLSTATE, if any.
    fn sql_state(&self) -> Option<Cow<'_, str>> {
        None
    }
}

impl Display for dyn LibraryError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.message())
    }
}

/// A wrapper around an error in the SQLx library.
pub struct SqlxError {
    sqlx_error: sqlx::Error,
}

impl SqlxError {
    fn raw(sqlx_error: sqlx::Error) -> Self {
        SqlxError {
            sqlx_error: sqlx_error,
        }
    }

    /// Wraps and takes ownership of a SQLx error.
    pub fn new(sqlx_error: sqlx::Error) -> Box<Self> {
        Box::new(SqlxError::raw(sqlx_error))
    }
}

impl LibraryError for SqlxError {
    fn message(&self) -> Cow<'_, str> {
        Cow::Owned(self.sqlx_error.to_string())
    }

    fn code(&self) -> ErrorCode {
        match self.sqlx_error {
            sqlx::Error::Configuration(_) => ErrorCode::ConfigurationError,
            sqlx::Error::Database(_) => ErrorCode::UnknownDatabaseError,
            sqlx::Error::Io(_) => ErrorCode::IoError,
            sqlx::Error::Tls(_) => ErrorCode::TlsError,
            sqlx::Error::Protocol(_) => ErrorCode::ProtocolError,
            sqlx::Error::RowNotFound => ErrorCode::RowNotFound,
            sqlx::Error::TypeNotFound { type_name: _ } => ErrorCode::TypeNotFound,
            sqlx::Error::ColumnIndexOutOfBounds { len: _, index: _ } => {
                ErrorCode::ColumnIndexOutOfBounds
            }
            sqlx::Error::ColumnNotFound(_) => ErrorCode::ColumnNotFound,
            sqlx::Error::ColumnDecode {
                index: _,
                source: _,
            } => ErrorCode::ColumnDecodeError,
            sqlx::Error::PoolTimedOut => ErrorCode::PoolTimedOut,
            sqlx::Error::PoolClosed => ErrorCode::PoolClosed,
            sqlx::Error::WorkerCrashed => ErrorCode::WorkerCrashed,
            _ => ErrorCode::UnknownSqlxError,
        }
    }

    fn sql_state(&self) -> Option<Cow<'_, str>> {
        match self.sqlx_error.as_database_error() {
            None => None,
            Some(err) => err.code(),
        }
    }
}

/// An error in the DataJoint library.
pub struct DataJointError {
    message: String,
    code: ErrorCode,
}

impl DataJointError {
    fn raw(message: &str, code: ErrorCode) -> Self {
        DataJointError {
            message: message.to_string(),
            code: code,
        }
    }

    /// Creates a new DataJoint error.
    ///
    /// The standard message for the error code is used as the error message.
    pub fn new(code: ErrorCode) -> Box<Self> {
        Box::new(DataJointError::raw(code.standard_message(), code))
    }

    /// Creates a new DataJoint error with a custom error message.
    pub fn new_with_message(message: &str, code: ErrorCode) -> Box<Self> {
        Box::new(DataJointError::raw(message, code))
    }
}

impl LibraryError for DataJointError {
    fn message(&self) -> Cow<'_, str> {
        Cow::Borrowed(&*self.message)
    }

    fn code(&self) -> ErrorCode {
        self.code
    }
}
