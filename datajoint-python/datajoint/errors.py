from .datajoint_core_lib import dj_core

error_codes = {
    dj_core.ErrorCode_Success: "Success",
    dj_core.ErrorCode_ConfigurationError: "Configuration Error",
    dj_core.ErrorCode_UnknownDatabaseError: "Unknown database Error",
    dj_core.ErrorCode_IoError: "I/O Error",
    dj_core.ErrorCode_TlsError: "TLS Error",
    dj_core.ErrorCode_ProtocolError: "Protocol Error",
    dj_core.ErrorCode_RowNotFound: "Error: Row Not Found",
    dj_core.ErrorCode_TypeNotFound: "Error: Type Not Found",
    dj_core.ErrorCode_ColumnIndexOutOfBounds: "Error: Column Index out of Bound",
    dj_core.ErrorCode_ColumnNotFound: "Column Not Found Error",
    dj_core.ErrorCode_ColumnDecodeError: "Column Decode Error",
    dj_core.ErrorCode_ValueDecodeError: "Value Decode Error",
    dj_core.ErrorCode_PoolTimedOut: "Error: Pool Timed Out",
    dj_core.ErrorCode_PoolClosed: "Error: Pool Closed",
    dj_core.ErrorCode_WorkerCrashed: "Error: Worker Crashed",
    dj_core.ErrorCode_UnknownSqlxError: "Unknown Sqlx Error",
    dj_core.ErrorCode_NotConnected: "Error: Not Connected",
    dj_core.ErrorCode_NoMoreRows: "Error: No More Rows",
    dj_core.ErrorCode_NullNotAllowed: "Error: Null Not Allowed",
    dj_core.ErrorCode_BufferNotEnough: "Error: Buffer Not Enough",
    dj_core.ErrorCode_InvalidNativeType: "Error: Invalid Native Type",
    dj_core.ErrorCode_InvalidCString: "Error: Invalid C Sting"
}


def datajoint_core_assert_success(error_code):
    if error_code != dj_core.ErrorCode_Success:
        error_message = error_codes[error_code]
        raise Exception(error_message)
