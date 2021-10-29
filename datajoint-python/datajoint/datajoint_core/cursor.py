from ._datajoint_core import ffi
from .datajoint_core_lib import dj_core
from .errors import datajoint_core_assert_success
from .table_row import TableRow


class Cursor:
    def __init__(self):
        self.native = ffi.new("Cursor**")
        self.native[0] = ffi.NULL

    def __del__(self):
        dj_core.cursor_free(self.native[0])

    def __iter__(self):
        return self

    def __next__(self):
        next_row = TableRow()
        err = dj_core.cursor_next(self.native[0], next_row.native)
        if err == dj_core.ErrorCode_Success:
            return next_row
        elif err == dj_core.ErrorCode_NoMoreRows:
            raise StopIteration
        else:
            datajoint_core_assert_success(err)
