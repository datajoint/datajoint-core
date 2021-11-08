"""
This module contains the `conn` function that provides access to a persistent connection in datajoint.
"""

from getpass import getpass

from .datajoint_core import Connection
from .settings import config


def conn(host=None, user=None, password=None, *, init_fun=None, reset=False, use_tls=None):
    """
    Returns a persistent connection object to be shared by multiple modules.
    If the connection is not yet established or reset=True, a new connection is set up.
    If connection information is not provided, it is taken from config which takes the
    information from dj_local_conf.json. If the password is not specified in that file
    datajoint prompts for the password.

    :param host: hostname
    :param user: mysql user
    :param password: mysql password
    :param init_fun: initialization function
    :param reset: whether the connection should be reset or not
    :param use_tls: TLS encryption option. Valid options are: True (required),
                    False (required no TLS), None (TLS prefered, default),
                    dict (Manually specify values per
                    https://dev.mysql.com/doc/refman/5.7/en/connection-options.html
                        # encrypted-connection-options).
    """
    if not hasattr(conn, 'connection') or reset:
        connection_config = {}
        connection_config["hostname"] = host if host is not None else config['database.host']
        connection_config["username"] = user if user is not None else config['database.user']
        connection_config["password"] = password if password is not None else config['database.password']
        if user is None:  # pragma: no cover
            connection_config["username"] = input(
                "Please enter DataJoint username: ")
        if password is None:  # pragma: no cover
            connection_config["password"] = getpass(
                prompt="Please enter DataJoint password: ")
        connection_config["use_tls"] = use_tls if use_tls is not None else config['database.use_tls']
        init_fun = init_fun if init_fun is not None else config['connection.init_function']
        conn.connection = Connection(connection_config)
    return conn.connection
