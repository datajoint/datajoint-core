from cffi import FFI
ffi = FFI()

ffi.cdef("""
            void conn();
            """)


#C = ffi.dlopen("..my-library/target/debug/my_library.dll")
connection = ffi.dlopen("../packages/datajoint-core-ffi-c/target/debug/connection.dll")

connection.conn()