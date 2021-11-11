# List of issues to address in the datajoint-core PR into datajoint-python

Most issues revolve arround functionality being used from pymysql that is not currently available in datajoint-core's client


### Errors on calls to `pymysql` for items methods or fields `datajoint-core` does not have

Direct errors from the code where `datajoint-python` calls a pymysql method that is not implemented yet. `client` and `self._conn` represent pymysql.

- [ ] client.err.ErrorName for checking errors
- [ ] self._conn.autocommit(True)
- [ ] self._conn.ping()

### Other issues to be address or decided upon

- [ ] placeholder arguments with the wrong form (`datajoint-python` uses tuples with all arguments)
- [ ] requests made on a cursor in datajont-python, vs connection with datajoint-core?
    - We have some flexibility here if we want to stick to `Connection.fetch_query` and `Connection.execute_query` or use the exposed `Cursor` and `Executor` methods.
- [ ] Replace uses of `execute_query` in existing `datajoint-python` with calls to `execute_query` and `fetch_query` based on what the query is doing.
- [ ] `sql_mode` and `charset` parameters passed to `pymysql.connect` are currently being ignored
- [ ] names of parameters such as `user` vs `username` and `password` vs `passwd` being used.