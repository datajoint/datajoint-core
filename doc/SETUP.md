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
