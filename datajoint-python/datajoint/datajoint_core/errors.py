from ._datajoint_core import ffi
from .datajoint_core_lib import dj_core


def datajoint_core_assert_success(error_code):
    if error_code != dj_core.ErrorCode_Success:
        error_message_raw = dj_core.datajoint_core_get_last_error_message()
        if error_message_raw == ffi.NULL:
            raise AssertionError("no error message provided by core library")

        error_message = ffi.string(error_message_raw).decode("utf-8")
        dj_core.datajoint_core_cstring_free(error_message_raw)
        raise AssertionError(error_message)
