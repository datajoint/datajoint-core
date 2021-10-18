#!/usr/bin/env python3
import datajoint as dj

connection = dj.conn(host="tutorial-db.datajoint.io", user="<username>", password="<password>", reset=False, use_tls=True)
result = connection.execute_query("""SELECT * from mouse""")
print(f'result from query is: {result}')