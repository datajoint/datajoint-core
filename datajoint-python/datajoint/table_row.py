from ._datajoint_core import ffi
from .datajoint_core_lib import dj_core
from .errors import datajoint_core_assert_success
from .table_column_ref import TableColumnRef
class TableRow:
    """
    TableRow class
    """

    def __init__(self):
        self.native = dj_core.table_row_new()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        dj_core.table_row_free(self.native)

    def __del__(self):
        dj_core.table_row_free(self.native)

    def is_empty(self):
        """
        Check if TableRow is empty
        """
        res = dj_core.table_row_is_empty(self.native)
        datajoint_core_assert_success(res)
        return bool(res)

    def columns(self):
        """
        Get all TableColumns
        """
        out_columns = ffi.new("TableColumnRef**")
        columns_size = ffi.new("size_t*")
        try:
            status = dj_core.table_row_columns(self.native, out_columns, columns_size)
            datajoint_core_assert_success(status)
            output = []
            for i in range(0, columns_size[0]):
            # Allows TableColumnRef Python wrapper to take ownership of the next element.
            # A little bit unsure of this dereferencing here, need to test.
                output.append(TableColumnRef(out_columns[i]))
                out_columns[i] = ffi.NULL
            return output
        finally:
            dj_core.table_row_columns_free(out_columns, columns_size)

    def column_count(self):
        """
        Get number of columns in TableRow
        """
        return dj_core.table_row_column_count(self.native)

    def column(self, index):
        """
        Get the column specified by name or ordinal
        """
        out_column = ffi.new("TableColumnRef*")
        # check if index is a ordinal or name
        status = dj_core.table_row_get_column_with_name(self.native, index.encode('utf-8'), out_column) if isinstance(index, str) else dj_core.table_row_get_column_with_ordinal(self.native, index, out_column)
        datajoint_core_assert_success(status)
        return TableColumnRef(out_column)
