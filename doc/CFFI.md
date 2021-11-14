# DataJoint Core C FFI
This document covers the overall design of the DataJoint Core C FFI.

# Design Patterns
This section provides a general list of design patterns followed in the C FFI.

- As much as possible, the layout of the C FFI crate should mirror the layout of the core library crate.
- Use opaque types for all DataJoint Core types. In other words, users of the C FFI should not be able to directly access attributes and methods on the types themselves. Appropraiate methods, getters, and setters should be provided for everything that should be exposed.
  - Thus, all functions should be named according to what object they correspond to. `Connection::reconnect` maps to `connection_reconnect`, `TableRow::columns` maps to `table_row_columns`, etc.
  - The first parameter of all C "methods" should represent the `this` or `self` parameter.
- Any function that can produce an error should return a 32-bit integer that represents the `ErrorCode`.
  - The last error is set by the function the error occurred in on a thread-local, static variable. The last error message and code can be accessed using the [appropriate C functions](../packages/datajoint-core-ffi-c/src/error/error.rs).
- Any value returned by a core library function or method should be returned to the caller of a C FFI function as an output parameter. Output parameters should come last in the parameter list.
  - Due to being opaque types, most output parameters should be double pointers, and this is reflected in many areas of the code. There is a [utility method for handling output parameters](../packages/datajoint-core-ffi-c/src/util/mem.rs) that handles allocating memory, freeing internal values, and reusing memory.
- As much as possible, Rust code should be responsible for memory management.
  - `malloc` and `free` should be avoided. Instead, `Box::into_raw` and `Box::from_raw` should be used for allocation and deallocation.
  - `*_new` methods should only be provided for types with public constructors. Types with non-public constructors should be constructed on the Rust side and given to the caller as an output parameter.
  - `*_free` methods should be provided for practically every type that is passed to the caller.
- C FFI should **never** `panic!`. This effectively means that every function should call the `try_*` version of core library methods and return the proper error to the caller.

## Connections
A `Connection` object represents a single connection to a SQL database. Every connection takes in a `ConnectionSettings` object, which specifies what database to connect to. These settings are exposed publicly on every connection, so the settings can be dynamically configured and one connection object can be used to connect to multiple databases over time. However, a connection object can only be connected to one database at a time.

```c
// Initialize settings.
ConnectionSettings* settings = connection_settings_new();
connection_settings_set_database_type(settings, DatabaseType_MySql);
connection_settings_set_username(settings, "username");
connection_settings_set_password(settings, "password");
connection_settings_set_hostname(settings, "tutorial-db.datajoint.io");
connection_settings_set_port(settings, 3306);
connection_settings_set_database_name(settings, "username_tutorial");

// Create connection.
Connection* conn = connection_new(settings);

// Connection takes ownership of settings, so set null here.
settings = NULL;

// Establish connection.
if (connection_connect(conn) != ErrorCode_Success) {
    printf("%s\n", datajoint_core_get_last_error_message());
    connection_free(my_conn);
}
```

## Executing Queries
Now that a connection is established with some SQL database, we want to execute queries against the database to manipulate or retrieve data.

### Types of Queries
There are two types of queries in the DataJoint Core Library.
- **Returning queries**, which return one or more rows.
- **Non-returning queries**, which return zero rows.

The type of query you wish to execute must be known at runtime, as they must be differentiated by the method you call.

```c
// Returning query.
// Returns a cursor that iterates over the rows of the query.
Cursor* cursor = NULL;
if (connection_fetch_query(conn, "select * from students;", NULL, &cursor) != ErrorCode_Success) {
    printf("%s\n", datajoint_core_get_last_error_message());
}

// Non-returning query.
// Returns the number of rows affected by the query.
uint64_t rows_affected;
if (connection_execute_query(conn, "insert into students (name, grade) values ('jackson', 12);", &rows_affected) != ErrorCode_Success) {
    printf("%s\n", datajoint_core_get_last_error_message());
}
```

### Cursors
A cursor is an iterator over a predetermined set of rows. Cursors are used to dynamically iterate over a set of rows that result from a returning query without fetching them all at once.

```c
// Create cursor for this query.
Cursor* cursor = NULL;
if (connection_fetch_query(conn, "select * from students;", NULL, &cursor) != ErrorCode_Success) {
    printf("%s\n", datajoint_core_get_last_error_message());
    return;
}

// At this point, the query has not executed.

// Loop through until no more rows are found.
for (;;) {
    int err = cursor_next(cursor, &next_row);
    if (err == ErrorCode_Success) {
        // Process next row.
    } else if (err == ErrorCode_NoMoreRows) {
        break;
    } else {
        printf("%s\n", datajoint_core_get_last_error_message());
        free_cursor(cursor);
        return;
    }
}
```

Alternatively, you can fetch the rest of the rows remaining in the cursor.
```c
TableRowVector* rows = NULL;
if (cursor_rest(cursor, &rows) != ErrorCode_Success) {
    printf("%s\n", datajoint_core_get_last_error_message());
}
```

It is important to note that it is impossible to iterate backwards through a cursor. If re-execution is desired, the cursor must be recreated using the query.

### Reading Results
Now that a query is stored or executed, it is time to actually read the results through the core library. A few objects are provided here.

- `TableRow` - A single row in a table.
- `TableColumnRef` - A reference to a single column in a table. All rows in the same table share a reference to the same table columns.

A tricky part about reading results over the C FFI is communicating typed data across multiple languages. Rust uses generics and enums, C uses void pointers, and user-level languages (such as Python) can be dynamically typed. Two decoding processes were created to solve this problem of language interop.

#### Buffer Decoding
The first type of decoding is called buffer decoding, which uses a caller-allocated buffer for sending decoded values from Rust to the caller. The caller allocates some buffer and passes it to the decoder. The core library will decode the value and attempt to move it into the buffer. It is then the caller's responsibility to move the decoded data out of the buffer, potentially converting it to some other type in the process.

For example, imagine the Python framework wants to read a string out of a single row. It creates a buffer that holds 50 bytes and sends it to the C FFI. The C FFI will place a null-terminated C string in that buffer. The Python caller then takes that string out and decodes it into a Python string, which can then be used in other parts of the framework.

#### Allocation Decoding
The second type of decoding is called allocation decoding, which uses a library-allocated buffer for sending decoded values from Rust to the caller. The caller does not create a buffer, but instead creates an `AllocatedDecodedValue` object, which wraps an internal buffer that is managed by Rust. It is still the caller's responsibility to move the decoded data out of the buffer, but the value inside will be properly deallocated when the wrapper is freed.

This method has the advantage of taking some of the memory management burden off of the caller. Furthermore, Rust can allocate the exact amount of memory needed for a single decoded value, which is useful for strings of varying lengths.

#### Which Decoding Method Should I Use?
Buffer decoding is primarily useful when the same allocation should be used for all decoded values. If a table is entirely the same data type, the same buffer can be used for decoding all values. If you are working with a large variety of data types, buffer decoding can be difficult to use, because it may not be known how large a buffer needs to be ahead of value decoding.

Allocation decoding makes up for this downfall of buffer decoding by letting Rust allocate memory after the value has been decoded. However, that allocation is freed and reallocated with every subsequent decode, so there will be a larger overhead. Allocation decoding is much more generic, but it comes with the cost of more allocations and deallocations over time.

Examples of both decoding methods can be found [here](../packages/datajoint-core-ffi-c/examples/decode.c).

### Placeholder Arguments
Each query function can optionally take in a `PlaceholderArgumentVector`, which is an ordered collection of placeholder arguments to be bound to a query.

Adding a placeholder argument to a `PlaceholderArgumentVector` is simple. You only need to pass in a pointer to some piece of data, the size (in bytes) of that data, and the native type of that data. The argument is then encoded into Rust.

```c
Cursor *cursor = NULL;

// Add placeholder argument.
PlaceholderArgumentVector *placeholders = placeholder_argument_vector_new();
char *sex = "F";
if (placeholder_argument_vector_add(placeholders, sex, 2, NativeTypeEnum_String, NULL) != ErrorCode_Success) {
    printf("Failed to add placeholder argument: %s\n", datajoint_core_get_last_error_message());
    placeholder_argument_vector_free(placeholders);
    connection_free(my_conn);
    return 1;
}

if (connection_fetch_query(my_conn, "select * from mouse where sex = ?;", placeholders, &cursor) != ErrorCode_Success) {
    printf("Failed to create cursor for query: %s\n", datajoint_core_get_last_error_message());
    connection_free(my_conn);
    return 1;
}

// Query takes ownership of placeholders vector and frees it, no free needed at this point.
placeholders = NULL;

// Use cursor.

cursor_free(cursor);
cursor = NULL;
```

Notice when and how the placeholders vector is freed. Once the vector is passed into `connection_fetch_query` successfully, it is consumed and owned by the query, and it no longer can be accessed from C. The pointer should immediately be set to `NULL` **with no free**, as it was already freed within the library code. 

## Handling Errors

Every function that can produce an error returns a 32-bit integer that represents the error code encountered. Integer constants are exposed over the C FFI for each error code so that errors can easily be checked by the caller.

It may be desired to receive more details on the error that occurred. For instance, SQLx database errors all use the same error code, but the message given is much more clear. The last library error encountered in the current thread is saved as a thread-local, static variable, and it can be accessed as needed.

```c
// Get last error message.
const char *datajoint_core_get_last_error_message();

// Get last error code.
int32_t datajoint_core_get_last_error_code();
```