from ._datajoint_core import ffi
from .datajoint_core_lib import dj_core
from .types import DataJointType 

class TableColumnRef:
    """
    TableColumnRef class
    """
    def __init__(self, native = None):
        if native is None:
            self.native = dj_core.table_column_ref_new();
        elif ffi.typeof(native) is ffi.typeof("TableColumnRef*"):
            self.native = native;
        else:
            raise ValueError("Invalid type for native pointer")
       
    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        dj_core.table_column_ref_free(self.native)

    def __del__(self):
        dj_core.table_column_ref_free(self.native)

    def ordinal(self):
        return dj_core.table_column_ref_ordinal(self.native)
    
    def name(self):
        return dj_core.table_column_ref_name(self.native)
    
    def type(self):
        return DataJointType(dj_core.table_column_ref_type(self.native))