# Setting Up DataJoint Core

## Build and Run

Build the rust library

```bash
cargo build
```

Build the python library
```bash
cd datajoint-python/datajoint
python datajoint/build_datajoint_core.py
```

Run the python tests

Import and use datajoint from the python interactive shell

```python
import datajoint as dj

connection = dj.conn("example@email.com", "Username123", "secretPassword", reset=False, use_tls=True)
cursor = connection.fetch_query("SELECT * FROM <database_name>.<Table> where <trait> = ?", '<trait-lit>')

try:
    l = list(cursor)
    for row in l:
        print(row.to_dict())
except AssertionError err:
    print(err)
```

Import FFI from cffi and create FFI object.

```python
from cffi import FFI
ffi = FFI()
```

Create space to define functions and open file for accessing Rust library. Change target
file path if needed. Functions can be defined from the ones found in packages\datajoint-core-ffi-c\datajoint-core-ffi-c.h. From there any function in the ffi.cdef can be used 

```python
ffi.cdef("""
    char *uuid_from_file(const char *bytes);

""")

C = ffi.dlopen("target\debug\datajoint_core_ffi_c.dll")

this = C.uuid_from_file("test.png".encode('utf-8'))
print(ffi.string(this))
```