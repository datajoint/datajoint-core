import sys
from os import path

prefix = {'win32': ''}.get(sys.platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
dirname = path.dirname(__file__)
library_file = path.join(
    dirname + '/../../../target/debug/' + prefix + 'datajoint_core_ffi_c' + extension)
header_file = path.join(
    dirname + '/../../../packages/datajoint-core-ffi-c/datajoint-core-ffi-c.h')
