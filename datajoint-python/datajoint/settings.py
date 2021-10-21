from ._datajoint_core import ffi
from .datajoint_core_lib import dj_core


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
            dj_core.connection_settings_free(self.native)

    # TODO(jackson-nestelroad): Type checking here for inputs and outputs.

    def __setitem__(self, setting, value):
        if setting.lower() in self.setters:
            if type(value) == str:
                value = value.encode("utf-8")
            self.setters[setting](self.native[0], value)
        else:
            raise Exception(f"No setting found with key: {setting}")

    def __getitem__(self, setting):
        if setting.lower() in self.getters:
            return self.getters[setting](self.native[0])
        else:
            raise Exception(f"No setting found with key: {setting}")


# Placeholders for setting default values into the config variable
# In the future this would be upadated with settings from environment
# variables similar to how it is done in 'datajoint-python/settings.py'
default_config = dict({
    "hostname": "ENV_HOSTNAME",
    "username": "ENV_USERNAME",
    "password": "ENV_PASSWORD",
    "port": 3306,
    "database_name": "ENV_DATABASE_NAME"
})
