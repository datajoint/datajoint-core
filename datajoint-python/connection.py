#!/usr/bin/env python3

import sys, ctypes
from ctypes import c_uint32, c_char_p, c_uint8, c_void_p, Structure, POINTER, c_bool


class ConnectionS(Structure):
    pass

prefix = {'win32': ''}.get(sys.platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
location = './packages/datajoint-core-ffi-c/target/debug/' + prefix + "core_ffi" + extension
lib = ctypes.cdll.LoadLibrary(location)

lib.connection_new.restype = POINTER(ConnectionS)

lib.connection_free.argtypes = (POINTER(ConnectionS), )


# connect_connect
lib.connection_connect.argtypes = (POINTER(ConnectionS), )
lib.connection_connect.restype = c_void_p

# connect_queryquery
lib.connection_query.argtypes = (POINTER(ConnectionS), c_char_p)
lib.connection_query.restype = c_uint32

class Connection:
    def __init__(self):
        self.obj = lib.connection_new()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        lib.connection_free(self.obj)

    def connect(self):
        lib.connection_connect(self.obj)

    def query(self, query):
        return lib.connection_query(self.obj, query.encode("utf-8"))

with Connection() as connection:

    connection.connect()
    connection.query(query="select 150")