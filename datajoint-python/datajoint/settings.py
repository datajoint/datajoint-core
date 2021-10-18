from ._datajoint_core import ffi
from .cffi_config import library_file
C = ffi.dlopen(library_file)

from .errors import check_error

class Config:

  setters = {
    'database_type': C.connection_settings_set_database_type,
    'username': C.connection_settings_set_username,
    'password': C.connection_settings_set_password,
    'hostname': C.connection_settings_set_hostname,
    'port': C.connection_settings_set_port,
    'database_name': C.connection_settings_set_database_name
  }

  getters = {
    'database_type': C.connection_settings_get_database_type,
    'username': C.connection_settings_get_username,
    'password': C.connection_settings_get_password,
    'hostname': C.connection_settings_get_hostname,
    'port': C.connection_settings_get_port,
    'database_name': C.connection_settings_get_database_name
  }

  def __init__(self):
    self._config = C.connection_settings_new()

  def __enter__(self):
    return self

  def __exit__(self):
    C.connection_free(self._config)

  def update(self, setting, value):
    if not setting.lower() in self.setters.keys():
      print(f"ERROR: could not update value for {setting} because it does not exist")
      return -1
    if type(value) == str:
      value = value.encode('utf-8')
    self.setters[setting](self._config, value)

  def get(self, setting):
    if not setting.lower() in self.getters.keys():
      print(f"ERROR: could not update value for {setting} because it does not exist")
      return -1
    return self.getters[setting](self._config)



config = Config()
config.update('hostname', 'ENV_HOSTNAME')
config.update('username', 'ENV_USERNAME')
config.update('password', 'ENV_PASSWORD')
config.update('port', 3306)
config.update('database_name', 'jonathan_tutorial')
# config.update('database_type', 200)