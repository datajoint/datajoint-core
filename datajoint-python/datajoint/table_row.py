from ._datajoint_core import ffi
from .cffi_config import library_file

lib = ffi.dlopen(library_file)


class TableRow:
    """
    TableRow class
    """

    def __init__(self):
        self._table_rows = lib.table_row_new()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        lib.table_row_free(self._table_rows)

    def is_empty(self):
        """
        Check if TableRow is empty
        """
        return bool(lib.table_row_is_empty(self._table_rows))

    def columns(self, out_columns, columns_size):
        """
        Get all TableColumns
        """
        try:
            status = lib.table_row_columns(self._table_rows, out_columns, columns_size)
        finally:
            lib.table_row_columns_free(out_columns, columns_size)

        return status

    def column_count(self):
        """
        Get number of columns in TableRow
        """
        return lib.table_row_column_count(self._table_rows)

    def column(self, index, out_column):
        """
        Get the column specified by name or ordinal
        """
        # check if index is a ordinal or name
        status = lib.table_row_get_column_with_name(self._table_rows, index, out_column) if isinstance(
            index, str) else lib.table_row_get_column_with_ordinal(self._table_rows, index, out_column)
        
        return status
