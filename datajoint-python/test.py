#!/usr/bin/env python3
import datajoint as dj

connection = dj.conn(host="tutorial-db.datajoint.io", user="jonathan",
                     password="testpassword123", database_name="jonathan_tutorial", reset=False, use_tls=True)


result = connection.execute_query_ph("""INSERT INTO mouse VALUES(?,?,?);""", 1, "2017-03-01", "M")
print(f'result from query is: {result}')

