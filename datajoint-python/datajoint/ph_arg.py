from .datajoint_core_lib import dj_core
from ._datajoint_core import ffi

class PlaceHolderArgumentVector:
    def __init__(self):
        self._ph_vec = dj_core.placeholder_argument_vector_new()

    def __enter__(self):
        return self

    def add(self, data):
        if isinstance(data, bytearray) or isinstance(data, bytes):
            c_data = ffi.new('unsigned char []', data)
            dj_core.placeholder_argument_vector_add(self._ph_vec, c_data, len(data), dj_core.DataJointType_Blob)
        if isinstance(data, str):
            c_data = ffi.new('char []', data.encode())
            dj_core.placeholder_argument_vector_add(self._ph_vec, c_data, len(data), dj_core.DataJointType_Date)
        if isinstance(data, float):
            p_data = ffi.new('float *', data)
            dj_core.placeholder_argument_vector_add(self._ph_vec, p_data, 0, dj_core.DataJointType_Float)
        if isinstance(data, int):
            p_data = ffi.new('int32_t *',data)
            dj_core.placeholder_argument_vector_add(self._ph_vec, p_data, 0, dj_core.DataJointType_Int)
        
    def __exit__(self, exc_type, exc_value, traceback):
        dj_core.placeholder_argument_vector_free(self._ph_vec)

    def print_args(self):
        dj_core.placeholder_argument_vector_print_args(self._ph_vec)

