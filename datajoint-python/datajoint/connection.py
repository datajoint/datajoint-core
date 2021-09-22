import os, sys, ctypes
from ctypes import c_uint32, c_char_p, c_uint8, c_void_p, Structure, POINTER, c_bool

class ConnectionS(Structure):
    pass

prefix = {'win32': ''}.get(sys.platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
dirname = os.path.dirname(__file__)
library_file = os.path.join(dirname + '/../../target/debug/' + prefix + 'corelib' + extension)
lib = ctypes.cdll.LoadLibrary(library_file)

lib.connection_new.restype = POINTER(ConnectionS)

lib.connection_free.argtypes = (POINTER(ConnectionS), )

# connect_connect
lib.connection_connect.argtypes = (POINTER(ConnectionS), c_char_p, c_char_p, c_char_p, c_bool, c_bool )
lib.connection_connect.restype = c_void_p

# connect_query
lib.connection_query.argtypes = (POINTER(ConnectionS), c_char_p)
lib.connection_query.restype = c_uint32

class Connection:
    def __init__(self, host, user, password, reset, use_tls):
        self.obj = lib.connection_new()
        self.connect(host, user, password, reset, use_tls)

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        lib.connection_free(self.obj)

    def connect(self, host, user, password, reset, use_tls):
        lib.connection_connect(self.obj, host.encode('utf-8') , user.encode('utf-8') , password.encode('utf-8'), reset, use_tls)

    def query(self, query):
        return lib.connection_query(self.obj, query.encode('utf-8'))

def conn(host=None, user=None, password=None, *, init_fun=None, reset=False, use_tls=None):
    conn.Connection = Connection(host, user, password, reset, use_tls)
    return conn.Connection

