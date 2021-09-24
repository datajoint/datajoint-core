import os, sys
from cffi import FFI
ffi = FFI()

prefix = {'win32': ''}.get(sys.platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
dirname = os.path.dirname(__file__)
library_file = os.path.join(dirname + '/../../target/debug/' + prefix + 'corelib' + extension)
header_file = os.path.join(dirname + '/../../packages/datajoint-core-ffi-c/datajoint-core-ffi-c.h')

with open(header_file, 'r') as f:
    headers = f.read()
    headers = headers.replace('Connection', 'void')
    ffi.cdef(headers)

C = ffi.dlopen(library_file)

class Connection:
    def __init__(self, host, user, password, reset, use_tls):
        self._conn = C.connection_new(host.encode('utf-8'), user.encode('utf-8'), password.encode('utf-8'), reset, use_tls)
        self.connect()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        C.connection_free(self._conn)

    def connect(self):
        C.connection_connect(self._conn)

    def raw_query(self, query):
        return C.connection_raw_query(self._conn, query.encode('utf-8'))

def conn(host=None, user=None, password=None, *, init_fun=None, reset=False, use_tls=None):
    conn.Connection = Connection(host, user, password, reset, use_tls)
    return conn.Connection

