"""
Settings for DataJoint.
"""
import os
import pprint
import collections

default = dict({
    'database.host': 'localhost',
    'database.password': None,
    'database.user': None,
    'database.port': 3306,
    'database.reconnect': True,
    'connection.init_function': None,
    'connection.charset': '',   # pymysql uses '' as default
    'loglevel': 'INFO',
    'safemode': True,
    'fetch_format': 'array',
    'display.limit': 12,
    'display.width': 14,
    'display.show_tuple_count': True,
    'database.use_tls': None,
    'enable_python_native_blobs': True,  # python-native/dj0 encoding support
})


class Config(collections.MutableMapping):

    instance = None

    def __init__(self, *args, **kwargs):
        if not Config.instance:
            Config.instance = Config.__Config(*args, **kwargs)
        else:
            Config.instance._conf.update(dict(*args, **kwargs))

    def __getattr__(self, name):
        return getattr(self.instance, name)

    def __getitem__(self, item):
        return self.instance.__getitem__(item)

    def __setitem__(self, item, value):
        self.instance.__setitem__(item, value)

    def __str__(self):
        return pprint.pformat(self.instance._conf, indent=4)

    def __repr__(self):
        return self.__str__()

    def __delitem__(self, key):
        del self.instance._conf[key]

    def __iter__(self):
        return iter(self.instance._conf)

    def __len__(self):
        return len(self.instance._conf)

    class __Config:
        def __init__(self, *args, **kwargs):
            self._conf = dict(default)
            self._conf.update(dict(*args, **kwargs))

        def __getitem__(self, key):
            return self._conf[key]

        def __setitem__(self, key, value):
            self._conf[key] = value


config = Config()

# override login credentials with environment variables
mapping = {k: v for k, v in zip(
    ('database.host', 'database.user', 'database.password',
     'external.aws_access_key_id', 'external.aws_secret_access_key',),
    map(os.getenv, ('DJ_HOST', 'DJ_USER', 'DJ_PASS',
                    'DJ_AWS_ACCESS_KEY_ID', 'DJ_AWS_SECRET_ACCESS_KEY',)))
    if v is not None}
config.update(mapping)
