from .datajoint_core_lib import dj_core
from ._datajoint_core import ffi


class PlaceholderArgumentVector:
    def __init__(self):
        self.native = dj_core.placeholder_argument_vector_new()

    def __del__(self):
        dj_core.placeholder_argument_vector_free(self.native)

    def add(self, data):
        if isinstance(data, bytearray) or isinstance(data, bytes):
            c_data = ffi.new("unsigned char[]", data)
            dj_core.placeholder_argument_vector_add(
                self.native, c_data, len(data), dj_core.NativeTypeEnum_Bytes, ffi.NULL)
        elif isinstance(data, str):
            c_data = ffi.new("char[]", data.encode("utf-8"))
            dj_core.placeholder_argument_vector_add(
                self.native, c_data, len(data), dj_core.NativeTypeEnum_String, ffi.NULL)
        elif isinstance(data, float):
            p_data = ffi.new("double*", data)
            dj_core.placeholder_argument_vector_add(
                self.native, p_data, 0, dj_core.NativeTypeEnum_Float64, ffi.NULL)
        elif isinstance(data, int):
            p_data = ffi.new("int32_t*", data)
            dj_core.placeholder_argument_vector_add(
                self.native, p_data, 0, dj_core.NativeTypeEnum_Int32, ffi.NULL)
        # TODO: Remove tuple instance when placeholder arguments format adjusted to math datajoint-python
        elif isinstance(data, tuple):
            pass
        else:
            print(f'type of data is' + str(type(data)))
            raise TypeError("unsupported placeholder argument type")
