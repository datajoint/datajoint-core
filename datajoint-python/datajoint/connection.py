from ._datajoint_core import ffi
from .cursor import Cursor
from .datajoint_core_lib import dj_core
from .errors import datajoint_core_assert_success
from .placeholders import PlaceholderArgumentVector
from .settings import Config, config


class Connection:
    def __init__(self, config):
        if not isinstance(config, Config):
            raise TypeError("config must be a connection configuration")
        self.native = dj_core.connection_new(config.native)
        config.native = ffi.NULL

    def __del__(self):
        dj_core.connection_free(self.native)

    def connect(self):
        err = dj_core.connection_connect(self.native)
        datajoint_core_assert_success(err)

    def disconnect(self):
        err = dj_core.connection_disconnect(self.native)
        datajoint_core_assert_success(err)

    def reconnect(self):
        err = dj_core.connection_reconnect(self.native)
        datajoint_core_assert_success(err)

    def execute_query(self, query, *args):
        out = ffi.new("uint64_t*")
        if len(args) == 0:
            err = dj_core.connection_execute_query(
                self.native, query.encode("utf-8"), ffi.NULL, out)
            datajoint_core_assert_success(err)
            return out[0]
        else:
            ph_args = PlaceholderArgumentVector()
            for arg in args:
                ph_args.add(arg)
            err = dj_core.connection_execute_query(
                self.native, query.encode("utf-8"), ph_args.native, out)
            ph_args.native = ffi.NULL
            return out[0]

    def fetch_query(self, query, *args):
        out = Cursor()
        if len(args) == 0:
            err = dj_core.connection_fetch_query(
                self.native, query.encode("utf-8"), ffi.NULL, out.native)
            datajoint_core_assert_success(err)
            return out
        else:
            ph_args = PlaceholderArgumentVector()
            for arg in args:
                ph_args.add(arg)
            err = dj_core.connection_fetch_query(
                self.native, query.encode("utf-8"), ph_args.native, out.native)
            datajoint_core_assert_success(err)
            ph_args.native = ffi.NULL
            return out


def conn(host=None, user=None, password=None, database_name=None, *, init_fun=None, reset=False, use_tls=None):
    if host is not None:
        config["hostname"] = host
    if user is not None:
        config["username"] = user
    if password is not None:
        config["password"] = password
    if database_name is not None:
        config["database_name"] = database_name
    connection = Connection(config)
    connection.connect()
    return connection
