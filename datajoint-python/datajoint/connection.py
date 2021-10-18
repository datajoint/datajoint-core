from .datajoint_core_lib import dj_core
from ._datajoint_core import ffi

from .settings import config
from .errors import datajoint_core_assert_success

class Connection:
    def __init__(self, config):
        self._conn = dj_core.connection_new(config)
        self.connect()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        dj_core.connection_free(self._conn)

    def connect(self):
        print("Attempting to make connection")
        err = dj_core.connection_connect(self._conn)
        datajoint_core_assert_success(err)

    def disconnect(self):
        err = dj_core.connection_disconnect(self._conn)
        datajoint_core_assert_success(err)

    def reconnect(self):
        err = dj_core.connection_reconnect(self._conn)
        datajoint_core_assert_success(err)

    def execute_query(self, query):
        out = ffi.new("uint64_t *")
        err = dj_core.connection_execute_query(self._conn, query.encode('utf-8'), out)
        datajoint_core_assert_success(err)
        return out[0]

    def fetch_query(self, query):
        pass
        # out = Cursor()
        # err = dj_core.connection_fetch_query(self._conn, query.encode('utf-8'), out)
        # datajoint_core_assert_success(err)
        # return out


def conn(host=None, user=None, password=None, database_name=None, *, init_fun=None, reset=False, use_tls=None):
    if host is not None:
        config["hostname"] = host
    if user is not None:
        config["username"] = user
    if password is not None:
        config["password"] = password
    if database_name is not None:
        config["database_name"] = database_name
    conn.Connection = Connection(config._config)
    return conn.Connection
