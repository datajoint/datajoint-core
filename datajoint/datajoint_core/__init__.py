"""
Wrapper library for core Rust library.

All references to the CFFI methods will take place inside this package.
"""

__author__ = "DataJoint Contributors"
__date__ = "November 18, 2021"

from .version import __version__
from . import connection_client

Connect = connect = connection_client.Connection

__all__ = [
    'connect',
    '__version__'
]
