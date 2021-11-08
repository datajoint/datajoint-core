# datajoint-core

Common DataJoint OSS framework

`./datajoint-python` is a temporary home for the light python wrapper to show a POC of how the Rust packages will operate.

## Build and Run

Build the rust library

```bash
cargo build
```

## Using the libarary
You can run the python from the interactive shell

```python
import datajoint as dj

connection = dj.conn(host="tutorial-db.datajoint.io", user=<username>,
                     password=<password>, database_name="jonathan_tutorial", reset=False, use_tls=True)

cursor = connection.fetch_query("select * from mouse where sex = ?", 'M')
try:
    l = list(cursor)
    for row in l:
        print(row.to_dict())
except AssertionError:
    print('failed :(')

connection.disconnect()
```
