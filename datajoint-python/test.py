#!/usr/bin/env python3

import datajoint as dj

connection = dj.conn("example@email.com", "Username123", "secretPassword", reset=False, use_tls=True)
result = connection.raw_query("SELECT STUFF FROM TABLE")
print(f'result from query is: {result}')