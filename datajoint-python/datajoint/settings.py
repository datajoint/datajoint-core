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

    def __init__(self):
        self.native = dj_core.connection_settings_new()

    def __del__(self):
        dj_core.connection_settings_free(self.native)

    # TODO(jackson-nestelroad): Type checking here for inputs and outputs.

    def __setitem__(self, setting, value):
        if setting.lower() in self.setters:
            if type(value) == str:
                value = value.encode("utf-8")
            self.setters[setting](self.native, value)
        else:
            raise Exception(f"No setting found with key: {setting}")

    def __getitem__(self, setting):
        if setting.lower() in self.getters:
            return self.getters[setting](self.native)
        else:
            raise Exception(f"No setting found with key: {setting}")


config = Config()

# Placeholders for setting default values into the config variable
# In the future this would be upadated with settings from environment
# variables similar to how it is done in 'datajoint-python/settings.py'
config["hostname"] = "ENV_HOSTNAME"
config["username"] = "ENV_USERNAME"
config["password"] = "ENV_PASSWORD"
config["port"] = 3306
config["database_name"] = "ENV_DATABASE_NAME"
