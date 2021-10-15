from cffi import FFI
from cffi_config import header_file
from os import path

ffi = FFI()

with open(header_file, 'r') as f:
    headers = f.read()
    ffi.cdef(headers)

ffi.set_source("datajoint._datajoint_core", None)

if __name__ == "__main__":
    ffi.compile()
