from ._datajoint_core import ffi
from .datajoint_core_lib import dj_core
from .datajoint_types import DataJointType


class TableColumnRef:
    """
    TableColumnRef class
    """

    def __init__(self, native=None, owning=True):
        self.native = ffi.new("TableColumnRef**")
        if native is None:
            self.native[0] = ffi.NULL
            self.owning = True
        elif ffi.typeof(native) is ffi.typeof("TableColumnRef*"):
            self.native[0] = native
            self.owning = owning
        else:
            raise ValueError("invalid type for native pointer")

    def __del__(self):
        if self.owning:
            dj_core.table_column_ref_free(self.native[0])

    def ordinal(self):
        return dj_core.table_column_ref_ordinal(self.native[0])

    def name(self):
        name = dj_core.table_column_ref_name(self.native[0])
        return ffi.string(name).decode("utf-8") if name else None

    def type(self):
        return DataJointType(dj_core.table_column_ref_type(self.native[0]))
