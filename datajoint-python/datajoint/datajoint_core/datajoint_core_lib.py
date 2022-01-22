from ._datajoint_core import ffi
from .cffi_config import library_file
dj_core = ffi.dlopen(library_file)