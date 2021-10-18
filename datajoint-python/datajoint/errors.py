from os import error
from ._datajoint_core import ffi
from .cffi_config import library_file

C = ffi.dlopen(library_file)

error_codes = {
  C.ErrorCode_Success: "Success",
  C.ErrorCode_ConfigurationError: "Configuration Error",
  C.ErrorCode_UnknownDatabaseError: "Unknown database Error",
  C.ErrorCode_IoError: "I/O Error",
  C.ErrorCode_TlsError: "TLS Error",
  C.ErrorCode_ProtocolError: "Protocol Error",
  C.ErrorCode_RowNotFound: "Error: Row Not Found",
  C.ErrorCode_TypeNotFound: "Error: Type Not Found",
  C.ErrorCode_ColumnIndexOutOfBounds: "Error: Column Index out of Bound",
  C.ErrorCode_ColumnNotFound: "Column Not Found Error",
  C.ErrorCode_ColumnDecodeError: "Column Decode Error",
  C.ErrorCode_ValueDecodeError: "Value Decode Error",
  C.ErrorCode_PoolTimedOut: "Error: Pool Timed Out",
  C.ErrorCode_PoolClosed: "Error: Pool Closed",
  C.ErrorCode_WorkerCrashed: "Error: Worker Crashed",
  C.ErrorCode_UnknownSqlxError: "Unknown Sqlx Error",
  C.ErrorCode_NotConnected: "Error: Not Connected",
  C.ErrorCode_NoMoreRows: "Error: No More Rows",
  C.ErrorCode_NullNotAllowed: "Error: Null Not Allowed",
  C.ErrorCode_BufferNotEnough: "Error: Buffer Not Enough",
  C.ErrorCode_InvalidNativeType: "Error: Invalid Native Type"
}

def check_error(error_code):
    error_message = error_codes[error_code]
    if error_code != C.ErrorCode_Success:
      raise Exception(error_message)