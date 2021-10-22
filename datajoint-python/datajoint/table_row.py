from ._datajoint_core import ffi
from .datajoint_core_lib import dj_core
from .errors import datajoint_core_assert_success
from .table_column_ref import TableColumnRef


class TableRow:
    """
    TableRow class
    """

    def __init__(self, native=None, owning=True):
        self.native = ffi.new("TableRow**")
        if native is None:
            self.native[0] = ffi.NULL
            self.owning = True
        elif ffi.typeof(native) is ffi.typeof("TableRow*"):
            self.native[0] = native
            self.owning = owning
        else:
            raise ValueError("invalid type for native pointer")

    def __del__(self):
        if self.owning:
            dj_core.table_row_free(self.native[0])

    def is_empty(self):
        """
        Check if TableRow is empty
        """
        res = dj_core.table_row_is_empty(self.native[0])
        return bool(res)

    def column_count(self):
        """
        Get number of columns in TableRow
        """
        return dj_core.table_row_column_count(self.native[0])

    def columns(self):
        """
        Get a list of all columns
        """
        # Not implemented due to a bug in cffi.
        # Variable-length arrays are not handled properly and result
        # in lost values.
        # Thus, all columns cannot be fetched at once at the moment.
        raise NotImplementedError()

    def column(self, index):
        """
        Get the column specified by name or ordinal
        """
        out_column = TableColumnRef()
        if type(index) == str:
            err = dj_core.table_row_get_column_with_name(
                self.native[0], index.encode('utf-8'), out_column.native)
            datajoint_core_assert_success(err)
        elif type(index) == int:
            err = dj_core.table_row_get_column_with_ordinal(
                self.native[0], index, out_column.native)
            datajoint_core_assert_success(err)
        else:
            raise TypeError("index must be a string or integer")
        return out_column

    def to_dict(self):
        try:
            # Single value wrapper for all decoded values.
            value = dj_core.allocated_decoded_value_new()

            result = dict()
            for i in range(self.column_count()):
                # Because we can't use columns(), we have to work on
                # the assumption that all columns are numbered properly via
                # their ordinal.
                col = self.column(i)
                col_name = col.name().decode("utf-8")
                err = dj_core.table_row_decode_to_allocation(
                    self.native[0], col.native[0], value)
                if err != dj_core.ErrorCode_Success:
                    result[col_name] = "DECODE FAILED"
                    continue

                # `raw_data` is a void* of length `data_size` bytes.
                raw_data = dj_core.allocated_decoded_value_data(value)
                data_size = dj_core.allocated_decoded_value_size(value)

                col_name = col.name().decode("utf-8")
                # Decode the value to a Python value.
                dj_type = dj_core.allocated_decoded_value_type(value)
                if dj_type == dj_core.NativeDecodedType_None:
                    result[col_name] = None
                elif dj_type == dj_core.NativeDecodedType_Int8:
                    result[col_name] = ffi.cast(
                        "int8_t*", raw_data)[0]
                elif dj_type == dj_core.NativeDecodedType_UInt8:
                    result[col_name] = ffi.cast(
                        "uint8_t*", raw_data)[0]
                elif dj_type == dj_core.NativeDecodedType_Int16:
                    result[col_name] = ffi.cast(
                        "int16_t*", raw_data)[0]
                elif dj_type == dj_core.NativeDecodedType_UInt16:
                    result[col_name] = ffi.cast(
                        "uint16_t*", raw_data)[0]
                elif dj_type == dj_core.NativeDecodedType_Int32:
                    result[col_name] = ffi.cast(
                        "int32_t*", raw_data)[0]
                elif dj_type == dj_core.NativeDecodedType_UInt32:
                    result[col_name] = ffi.cast(
                        "uint32_t*", raw_data)[0]
                elif dj_type == dj_core.NativeDecodedType_String:
                    result[col_name] = ffi.string(
                        ffi.cast("char*", raw_data), data_size).decode('utf-8')
                elif dj_type == dj_core.NativeDecodedType_Float32:
                    result[col_name] = ffi.cast(
                        "float*", raw_data)[0]
                elif dj_type == dj_core.NativeDecodedType_Float64:
                    result[col_name] = ffi.cast(
                        "double*", raw_data)[0]
                elif dj_type == dj_core.NativeDecodedType_Bytes:
                    result[col_name] = ffi.unpack(
                        ffi.cast("unsigned char*", raw_data), data_size)
                else:
                    raise AssertionError("decoded value has invalid type name")

            return result

        finally:
            dj_core.allocated_decoded_value_free(value)
