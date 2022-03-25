from cffi import FFI
from ctypes import *

ffi = FFI()



ffi.cdef("""
         
        struct Vec_u8 *packInt(int64_t this_);

        struct Vec_u8 *packFloat(double this_);
        
        struct Vec_u8 *packString(struct String this_);

        struct Vec_u8 *packBool(bool this_);

        void unpack(struct Vec_u8 *this_);

         """)


C = ffi.dlopen("target/debug/datajoint_core_ffi_c.dll")

this = C.packInt(12)

print("int is: ")
print(this)
C.unpack(this)

print()

print("float is: ")
this = C.packFloat(3.2343584785385744)
print(this)
C.unpack(this)

print()

#print("String is: ")
#this = C.packString("hello world")
#print(this)
#C.unpack(this)
#

print()

print("Boolean is: ")
this = C.packBool(False)
print(this)
C.unpack(this)


exit()