from ._datajoint_core import ffi
from .cursor import Cursor
from .datajoint_core_lib import dj_core
from .errors import datajoint_core_assert_success
from .placeholders import PlaceholderArgumentVector
from .connection_config import Config


class Connection:
    def __init__(self, default_config=None, sql_mode=None, charset=None, init_command=None):
        print(f'Default config is')
        print(default_config)
        self.native = dj_core.connection_new(dj_core.connection_settings_new())
        self.config = Config(
            native=dj_core.connection_get_settings(self.native),
            owning=False
        )
        if default_config is not None:
            self.config.update(default_config)
        self.connect()

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

        ph_args = PlaceholderArgumentVector()
        for arg in args:
            ph_args.add(arg)
        err = dj_core.connection_execute_query(
            self.native, query.encode("utf-8"), ph_args.native, out)
        ph_args.native = ffi.NULL
        datajoint_core_assert_success(err)
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
            ph_args.native = ffi.NULL
            datajoint_core_assert_success(err)
            return out
