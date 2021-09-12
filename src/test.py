from cffi import FFI
ffi = FFI()

ffi.cdef("""
    int doubleit(int);
""")


#C = ffi.dlopen("..my-library/target/debug/my_library.dll")
C = ffi.dlopen("C:/Users/Jonny/Desktop/datajoint/my-library/target/debug/my_library.dll")

print(C.doubleit(9))