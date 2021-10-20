from .datajoint_core_lib import dj_core
from ._datajoint_core import ffi

class PlaceHolderArgumentVector:
    def __init__(self):
        self.native = dj_core.placeholder_argument_vector_new()

    def __del__(self):
        dj_core.placeholder_argument_vector_free(self.native)

    def add(self, data):
        if isinstance(data, bytearray) or isinstance(data, bytes):
            c_data = ffi.new('unsigned char[]', data)
            dj_core.placeholder_argument_vector_add(
                self.native, c_data, len(data), dj_core.DataJointType_Blob)
        elif isinstance(data, str):
            c_data = ffi.new('char[]', data.encode())
            dj_core.placeholder_argument_vector_add(
                self.native, c_data, len(data), dj_core.DataJointType_Date)
        elif isinstance(data, float):
            p_data = ffi.new('float*', data)
            dj_core.placeholder_argument_vector_add(
                self.native, p_data, 0, dj_core.DataJointType_Float)
        elif isinstance(data, int):
            p_data = ffi.new('int32_t*', data)
            dj_core.placeholder_argument_vector_add(
                self.native, p_data, 0, dj_core.DataJointType_Int)
        else:
            raise TypeError("unsupported placeholder argument type")
