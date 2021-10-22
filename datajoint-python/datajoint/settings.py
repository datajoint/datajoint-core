from .datajoint_core_lib import dj_core
from ._datajoint_core import ffi


class Config:

    setters = {
        'database_type': dj_core.connection_settings_set_database_type,
        'username': dj_core.connection_settings_set_username,
        'password': dj_core.connection_settings_set_password,
        'hostname': dj_core.connection_settings_set_hostname,
        'port': dj_core.connection_settings_set_port,
        'database_name': dj_core.connection_settings_set_database_name
    }

    getters = {
        'database_type': dj_core.connection_settings_get_database_type,
        'username': dj_core.connection_settings_get_username,
        'password': dj_core.connection_settings_get_password,
        'hostname': dj_core.connection_settings_get_hostname,
        'port': dj_core.connection_settings_get_port,
        'database_name': dj_core.connection_settings_get_database_name
    }

    database_type = {
        "MySQL": dj_core.DatabaseType_MySql,
        "Postgres": dj_core.DatabaseType_Postgres,
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
        if setting.lower() in self.setters:
            if value in self.database_type:
                value = self.database_type[value]
            if type(value) == str:
                value = value.encode("utf-8")
            self.setters[setting](self.native[0], value)
        else:
            raise Exception(f"No setting found with key: {setting}")

    def __getitem__(self, setting):
        if setting.lower() in self.getters:
            val = self.getters[setting](self.native[0])
            if type(val) == int:
                return val
            elif ffi.typeof(val) == ffi.typeof("char *"):
                return ffi.string(val).decode("utf-8")
        else:
            raise Exception(f"No setting found with key: {setting}")

    def __repr__(self):
        settings = {setting: self[setting] for setting in self.getters}
        if settings["database_type"] == dj_core.DatabaseType_MySql:
            settings["database_type"] = "MySql"
        else:
            settings["database_type"] = "Postgres"
        rep = "Database Settings:\n"
        for setting in settings:
            rep += f"{setting}: {settings[setting]}\n"
        return rep

    def update(self, mapping):
        for key in mapping:
            self[key] = mapping[key]


# Placeholders for setting default values into the config variable
# In the future this would be upadated with settings from environment
# variables similar to how it is done in 'datajoint-python/settings.py'
import os
from dotenv import load_dotenv
load_dotenv()

default_config = {k: v for k, v in zip(
    ("hostname", "username", "password",
     "port", "database_name",),
    map(os.getenv, ("DJ_HOST", "DJ_USER", "DJ_PASS",
                    "PORT", "DB_NAME")))
            if v is not None}
default_config["port"] = int(default_config["port"])
