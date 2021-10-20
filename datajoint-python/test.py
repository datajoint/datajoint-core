#!/usr/bin/env python3
import datajoint as dj

<<<<<<< HEAD
connection = dj.conn(host="tutorial-db.datajoint.io", user="<username>",
                     password="<password>", database_name="jonathan_tutorial", reset=False, use_tls=True)
result = connection.execute_query("""SELECT * from mouse""")
=======

connection = dj.conn(host="tutorial-db.datajoint.io", user="jonathan",
                     password="testpassword123", database_name="jonathan_tutorial", reset=False, use_tls=True)

cursor = connection.fetch_query("select * from mouse")
try:
    l = list(cursor)
    for row in l:
        print(row.to_dict())
except AssertionError:
    print('failed :(')

connection.disconnect()
>>>>>>> d8385d5cc51b44b5507217737d9b76ea088f643f
