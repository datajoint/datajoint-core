import os
import sys
from cffi import FFI
ffi = FFI()

prefix = {'win32': ''}.get(sys.platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
dirname = os.path.dirname(__file__)
library_file = os.path.join(
    dirname + '/../../target/debug/' + prefix + 'corelib' + extension)
header_file = os.path.join(
    dirname + '/../../packages/datajoint-core-ffi-c/datajoint-core-ffi-c.h')

with open(header_file, 'r') as f:
    headers = f.read()
    ffi.cdef(headers)

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
