[![Coverage Status](https://coveralls.io/repos/github/EdwardGarmon/datajoint-core/badge.svg?branch=integration-tests)](https://coveralls.io/github/EdwardGarmon/datajoint-core?branch=integration-tests)
# datajoint-core

Common DataJoint OSS framework 

`./datajoint-python` is a temporary home for the light python wrapper to show a POC of how the Rust packages will operate.

## Build and Run

Build the rust library

```bash
cargo build
```

Run the python tests

```bash
python datajoint-python/test.py
```

output

```
Connected to database: Host: example@email.com User: Username123 Password: secretPassword Reset: false use_TLS: true
Making query from rust library: SELECT STUFF FROM TABLE
result from query is: 0
```

You can also test from the python interactive shell

```python
import datajoint as dj

connection = dj.conn("example@email.com", "Username123", "secretPassword", reset=False, use_tls=True)
connection.raw_query("SELECT STUFF FROM TABLE")
```
