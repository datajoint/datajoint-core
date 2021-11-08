import sys
from os import path

prefix = {'win32': ''}.get(sys.platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
dirname = path.dirname(__file__)
relative_path = '/../../../datajoint-core'

library_file = path.join(
    dirname + relative_path
    + '/target/debug/'
    + prefix
    + 'corelib'
    + extension)

header_file = path.join(
    dirname + relative_path
    + '/packages/datajoint-core-ffi-c/datajoint-core-ffi-c.h'
    + prefix)
