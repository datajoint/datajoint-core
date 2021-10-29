import os
from enum import Enum
from dotenv import load_dotenv
from .datajoint_core_lib import dj_core
from ._datajoint_core import ffi


def free_and_decode_string(value):
    dj_core.datajoint_core_cstring_free(value)
    return ffi.string(value).decode("utf-8")


class DatabaseType(Enum):
    MySQL = dj_core.DatabaseType_MySql
    Postgres = dj_core.DatabaseType_Postgres

    @classmethod
    def from_str(cls, label):
        if label.lower() == "mysql":
            return cls.MySQL.value
        if label.lower() == "postgres":
            return cls.Postgres.value
        raise NotImplementedError


class ConnectionSetting:
    def __init__(self, getter, setter, ffi_type):
        self.getter = getter
        self.setter = setter
        self.ffi_type = ffi_type

    def set_value(self, native, value):
        encode_methods = {
            int: int,
            str: lambda val: val.encode("utf-8"),
            DatabaseType: DatabaseType.from_str
        }
        self.setter(native, encode_methods[self.ffi_type](value))

    def get_value(self, native):
        decode_methods = {
            int: lambda val: val,
            # TODO(jonathan-hocevar): Free's value at the wrong time.
            str: lambda val: free_and_decode_string(val),
            DatabaseType: lambda val: DatabaseType(val).name
        }
        return decode_methods[self.ffi_type](self.getter(native))


class Config:
    _fields = {
        "database_type": ConnectionSetting(
            getter=dj_core.connection_settings_get_database_type,
            setter=dj_core.connection_settings_set_database_type,
            ffi_type=DatabaseType
        ),
        "hostname": ConnectionSetting(
            getter=dj_core.connection_settings_get_hostname,
            setter=dj_core.connection_settings_set_hostname,
            ffi_type=str
        ),
        "username": ConnectionSetting(
            getter=dj_core.connection_settings_get_username,
            setter=dj_core.connection_settings_set_username,
            ffi_type=str
        ),
        "password": ConnectionSetting(
            getter=dj_core.connection_settings_get_password,
            setter=dj_core.connection_settings_set_password,
            ffi_type=str
        ),
        "database_name": ConnectionSetting(
            getter=dj_core.connection_settings_get_database_name,
            setter=dj_core.connection_settings_set_database_name,
            ffi_type=str
        ),
        "port": ConnectionSetting(
            getter=dj_core.connection_settings_get_port,
            setter=dj_core.connection_settings_set_port,
            ffi_type=int
        )
    }

    def __init__(self, native=None, owning=True):
        self.native = ffi.new("ConnectionSettings**")
        if native is None:
            self.native[0] = ffi.NULL
            self.owning = True
        elif ffi.typeof(native) is ffi.typeof("ConnectionSettings*"):
            self.native[0] = native
            self.owning = owning
        else:
            raise ValueError("invalid type for native pointer")

    def __del__(self):
        if self.owning:
            dj_core.connection_settings_free(self.native[0])

    # TODO(jackson-nestelroad): Type checking here for inputs and outputs.

    def __setitem__(self, setting, value):
        field = self._fields[setting]
        field.set_value(self.native[0], value)

    def __getitem__(self, setting):
        field = self._fields[setting]
        return field.get_value(self.native[0])

    def get_settings(self):
        settings = dict()
        for name, setting in self._fields.items():
            settings[name] = setting.get_value(self.native[0])
        return settings

    def __repr__(self):
        rep = "Database Settings:\n"
        for setting, value in self.get_settings().items():
            rep += f"{setting}: {value}\n"
        return rep

    def update(self, mapping):
        for key in mapping:
            self[key] = mapping[key]


# Placeholders for setting default values into the config variable
# In the future this would be upadated with settings from environment
# variables similar to how it is done in 'datajoint-python/settings.py'
load_dotenv()

default_config = {k: v for k, v in zip(
    ("hostname", "username", "password",
     "port", "database_name",),
    map(os.getenv, ("DJ_HOST", "DJ_USER", "DJ_PASS",
                    "PORT", "DB_NAME")))
    if v is not None}
default_config["port"] = int(default_config["port"])
