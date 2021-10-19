from ._datajoint_core import ffi
from .datajoint_core_lib import dj_core
from .errors import datajoint_core_assert_success
class TableRow:
    """
    TableRow class
    """

    def __init__(self):
        self._table_rows = dj_core.table_row_new()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        dj_core.table_row_free(self._table_rows)

    def __del__(self):
        dj_core.table_row_free(self._table_rows)

    def is_empty(self):
        """
        Check if TableRow is empty
        """
        res = dj_core.table_row_is_empty(self._table_rows)
        datajoint_core_assert_success(res)
        return bool(res)

    def columns(self):
        """
        Get all TableColumns
        """
        out_columns = ffi.new("TableColumnRef*")
        columns_size = ffi.new("int*")
        try:
            status = dj_core.table_row_columns(self._table_rows, out_columns , columns_size)
        finally:
            dj_core.table_row_columns_free(out_columns, columns_size)
        datajoint_core_assert_success(status)
        return out_columns

    def column_count(self):
        """
        Get number of columns in TableRow
        """
        return dj_core.table_row_column_count(self._table_rows)

    def column(self, index):
        """
        Get the column specified by name or ordinal
        """
        out_column = ffi.new("TableColumnRef*")
        # check if index is a ordinal or name
        status = dj_core.table_row_get_column_with_name(self._table_rows, index.encode('utf-8'), out_column) if isinstance(index, str) else dj_core.table_row_get_column_with_ordinal(self._table_rows, index, out_column)
        datajoint_core_assert_success(status)
        return out_column
