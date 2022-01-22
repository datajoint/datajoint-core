from ._datajoint_core import ffi
from .datajoint_core_lib import dj_core
from .table_row import TableRow


class TableRowVector:
    def __init__(self, native=None, owning=True):
        self.native = ffi.new("TableRowVector**")
        if native is None:
            self.native[0] = ffi.NULL
            self.owning = True
        elif ffi.typeof(native) is ffi.typeof("TableRowVector*"):
            self.native[0] = native
            self.owning = owning
        else:
            raise ValueError("invalid type for native pointer")

    def __del__(self):
        if self.owning:
            dj_core.table_row_vector_free(self.native[0])

    def get(self, index):
        row = dj_core.table_row_vector_get(self.native, index)
        if row:
            return TableRow(native=row, owning=False)
        return None

    def size(self):
        return dj_core.table_row_vector_size(self.native)
