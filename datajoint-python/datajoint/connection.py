from ._datajoint_core import ffi
from .cursor import Cursor
from .datajoint_core_lib import dj_core
from .errors import datajoint_core_assert_success
from .placeholders import PlaceholderArgumentVector
from .settings import Config, default_config


class Connection:
    def __init__(self):
        self.native = dj_core.connection_new(dj_core.connection_settings_new())
        self.config = Config(
            native=dj_core.connection_get_settings(self.native), owning=False)

        # TODO(jackson-nestelroad): Probably don't do this here, do it in some
        # higher-level layer.
        for key in default_config:
            self.config[key] = default_config[key]

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


def conn(host=None, user=None, password=None, database_name=None, database_type=None, *, init_fun=None, reset=False, use_tls=None):
    connection = Connection()
    if host is not None:
        connection.config["hostname"] = host
    if user is not None:
        connection.config["username"] = user
    if password is not None:
        connection.config["password"] = password
    if database_name is not None:
        connection.config["database_name"] = database_name
    if database_type is not None:
        connection.config["database_type"] = database_type
    connection.connect()
    return connection