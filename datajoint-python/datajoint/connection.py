from ._datajoint_core import ffi
from .cffi_config import library_file
C = ffi.dlopen(library_file)

from .settings import config
from .errors import check_error

class Connection:
    def __init__(self, config):
        self._conn = C.connection_new(config)
        self.connect()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        C.connection_free(self._conn)

    def connect(self):
        print("Attempting to make connection")
        err = C.connection_connect(self._conn)
        check_error(err)

    def disconnect(self):
        err = C.connectin_disconnect(self._conn)
        check_error(err)

    def reconnect(self):
        self.disconnect()
        self.connect()

    def execute_query(self, query):
        out = ffi.new("uint64_t *")
        err = C.connection_execute_query(self._conn, query.encode('utf-8'), out)
        check_error(err)
        out = out[0]
        print(f'rows found: {out}')

def conn(host=None, user=None, password=None, database_name=None, *, init_fun=None, reset=False, use_tls=None):
    if host is not None:
        config.update("hostname", host)
    if user is not None:
        config.update("username", user)
    if password is not None:
        config.update("password", password)
    if database_name is not None:
        config.update("database_name", database_name")
    conn.Connection = Connection(config._config)
    return conn.Connection
