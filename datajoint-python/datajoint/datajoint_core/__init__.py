"""
Wrapper library for core Rust library.

All references to the CFFI methods will take place inside this package.
"""

__author__ = "DataJoint Contributors"
__date__ = "November 18, 2021"
__all__ = ['__author__', '__version__', 'Connection']

from .version import __version__
from .connection_client import Connection
