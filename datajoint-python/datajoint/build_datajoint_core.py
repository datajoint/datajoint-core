from cffi import FFI
import cffi_config

ffi = FFI()

with open(cffi_config.header_file, 'r') as file:
    headers = file.read()
    ffi.cdef(headers)

ffi.set_source("datajoint._datajoint_core", None)

if __name__ == "__main__":
    ffi.compile()
