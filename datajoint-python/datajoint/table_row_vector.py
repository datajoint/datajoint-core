from ._datajoint_core import ffi
from .cffi_config import library_file

lib = ffi.dlopen(library_file)


class TableRowVector:
    def __init__(self):
        self._table_rows = lib.table_row_vector_new()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        lib.table_row_vector_free(self._table_rows)

    def get(self, index):
        return lib.table_row_vector_get(self._table_rows, index)

    def size(self):
        return lib.table_row_vector_size(self._table_rows)
