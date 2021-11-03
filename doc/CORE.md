# DataJoint Core Library
This document covers the overall design of the DataJoint Core Library.

# Purpose
DataJoint is an open-source software framework for managing and sharing scientific data pipelines in Python and MATLAB. DataJoint allows scientists to employ the benefits of relational databases through a layer of abstraction.

The DataJoint Core Library is a low-level Rust package (commonly referred to as a “crate”) for shared code across DataJoint client libraries written in high-level programming languages (such as Python and MATLAB). The DataJoint Core Library exposes a C FFI to allow DataJoint frameworks in different programming languages to use the same core code. For example, any DataJoint framework will need to connect to a SQL database. Rather than writing connection code in the user-level programming language, this connection code can be written in the core library to be used by all user-level DataJoint frameworks.

DataJoint currently has no such core library for shared code, forcing DataJoint developers to maintain multiple frameworks across multiple programming languages. The team currently only supports two languages—Python and MATLAB—but the team is forced to write and maintain a lot of duplicate code, decreasing developer productivity and overall project maintainability. Furthermore, there is a desire to potentially create a DataJoint framework for other programming languages, such as JavaScript or Java, but these new frameworks would also have to rewrite all of the common code, making new features much harder to add across the ecosystem. The DataJoint Core Library aims to solve this problem by moving all shared code to a common library that can be called by any language.

DataJoint can be considered an object-relational mapping (ORM) framework, but ORMs are typically written for a single programming language with no cross-language support considered. Low-level libraries are provided for working with SQL databases, but these libraries are typically flavor-specific (such as MySQL or PostgreSQL). The DataJoint Core Library project is a unique attempt to build a low-level library with a C FFI that works with flavor-agnostic SQL databases that can be used to create a shared ORM experience across multiple programming languages. The DataJoint Core Library aims to make the future of DataJoint development much more efficient while also opening the door to build new DataJoint frameworks in other popular programming languages.

# Design and Usage
This section demonstrates how the DataJoint Core Library can be used as a Rust crate. Implementers of user-level DataJoint frameworks (such as those in Python or MATLAB) should look at the [DataJoint Core C FFI](../packages/datajoint-core-ffi-c) instead. 

## Connections
A `Connection` object represents a single connection to a SQL database. Every connection takes in a `ConnectionSettings` object, which specifies what database to connect to. These settings are exposed publicly on every connection, so the settings can be dynamically configured and one connection object can be used to connect to multiple databases over time. However, a connection object can only be connected to one database at a time.

```rs
let settings = ConnectionSettings::new();
settings.database_type = DatabaseType::Postgres;
// Set username, password, hostname, port, database_name as necessary.

let conn = Connection::new(settings);

// A connection is not officially established until connect() is manually called.
conn.connect().unwrap();

// Dynamic reconfiguration.
conn.settings.database_type = DatabaseType::MySql;
conn.reconnect().unwrap();

// Disconnect as desired.
conn.disconnect().unwrap();
```

## Executing Queries
Now that a connection is established with some SQL database, we want to execute queries against the database to manipulate or retrieve data.

### Types of Queries
There are two types of queries in the DataJoint Core Library.
- **Returning queries**, which return one or more rows.
- **Non-returning queries**, which return zero rows.

The type of query you wish to execute must be known at runtime, as they must be differentiated by the method you call.

```rs
// Returning query.
// Returns a cursor that iterates over the rows of the query.
let mut cursor = conn.fetch_query("select * from students;");

// Non-returning query.
// Returns the number of rows affected by the query.
let rows_affected = conn.execute_query("insert into students (name, grade) values ('jackson', 12);");
```

### Cursors
A cursor is an iterator over a predetermined set of rows. Cursors are used to dynamically iterate over a set of rows that result from a returning query without fetching them all at once.

```rs
// Create cursor for this query.
let mut cursor = conn.fetch_query("select * from students;");

// At this point, the query has not executed.

// Loop through until no more rows are found.
loop {
    match cursor.try_next() {
        Err(err) => {
            if err.code() == ErrorCode::NoMoreRows {
                break;
            } else {
                panic!(err.message());
            }
        }
        Ok(row) => {
            // Process next row.
        }
    }
}
```

Alternatively, you can fetch the rest of the rows remaining in the cursor.
```rs
let rows: Vec<TableRow> = cursor.rest();
```

It is important to note that it is impossible to iterate backwards through a cursor. If re-execution is desired, the cursor must be recreated using the query.

### Executors
Although there are two high-level methods for executing both types of queries over a `Connection` object, there is another API for executing queries if desired.

An executor is an object used to interact with a database by executing queries. Its sole purpose is to execute queries.

```rs
// Create an executor over the current connection.
let executor = conn.executor();
```

Executors have a lot of the same methods, but things are a big more flexible.
```rs
// Execute a non-returning query.
let rows_affected: u64 = executor.execute("insert into students (name, grade) values ('jackson', 12);");

// Fetch a single row, ignoring the rest.
let single_row: TableRow = executor.fetch_one("select * from students;");

// Fetch all rows at once without a cursor.
let all_rows: Vec<TableRow> = executor.fetch_all("select * from students;");

// Create a cursor for the given query.
let cursor = executor.cursor("select * from students;");
```

All of the query methods on the `Connection` object are actually shortcuts for using executors.

As of now, an executor only wraps a connection pool that is tied to a single `Connection` object. However, the purpose of this object is to provide a stable API for executing queries. In the future, SQL transactions should use this same interface to execute queries in a single transaction before committing it.

### Reading Results
Now that a query is stored or executed, it is time to actually read the results through the core library. A few objects are provided here.

- `TableRow` - A single row in a table.
- `TableColumnRef` - A reference to a single column in a table. All rows in the same table share a reference to the same table columns.
- `TableColumn` - An owned version of a `TableColumnRef`.

A single value can be fetched by column name or ordinal. If you know the name of the columns being returned, fetching a value is very easy.

```rs
let cursor: TableRow = conn.fetch_query("select name from students;");
let row = cursor.next();

// Rust code uses generics for decoding, so the type name must be given.
let name: String = row.get("name");
```

Obviously, using the column name or index by hand is inefficient and not very generalized. Column objects are provided for this purpose of accessing column names. However, it is important to discuss how values are decoded internally for this next part.

Individual values are decoded depending on the generic type parameter given to the `get` method. Each type must implement the proper type traits for decoding. Because of this requirement, generalized decoding of any column using the same code is impossible, because different columns will likely require different types. Thus, a decoding API is exposed on the `TableRow` object as well that implements decoding based on DataJoint standards.

```rs
let cursor: TableRow = conn.fetch_query("select name from students;");
let row = cursor.next();

for col in row.columns() {
    // NativeType is an enum around supported native types, such as
    // integers, floats, character strings, and byte strings.
    // 
    // A match statement should be used to unwrap this value.
    let value: NativeType = row.decode(col);

    // While `decode` considers null values an error, `decode_optional`
    // returns null values as `Option::None`.
    let value: Option<NativeType> = row.decode_optiona(col);
}
```

The [decoding process](../packages/datajoint-core/src/types/decode.rs) is a bit complicated, but it essentially follows these steps:

1. Get the DataJoint type of the given column using `TableColumnRef::type_name`.
2. Decode the column to a surely supported native type using `TableRow::try_get`. If a type is not supported differently depending on the SQL flavor, enum unwrapping is equired to work with the row value.
3. Wrap the decoded value in the proper `NativeType` variant.

The decode functionality is critically important to the C FFI due to the limitations of C functions and for interoperability with dynamically-typed languages.

### Placeholder Arguments
Placeholder arguments are an important feature for protecting against SQL injection. Working with placeholder arguments can be imagined as the opposite of the decode process described above: a collection of `NativeType` variants (wrapping a corrsponding value) is encoded into a query prior to execution. Thus, the same enum is used here.

A placeholder version of pretty much every query-executing method is provided using the suffix `_ph`.

```rs
// This type implements the `PlaceholderArgumentCollection` trait.
let placeholders: Vec<NativeType> = vec![NativeType::Int32(12)];
let mut cursor = conn.fetch_query_ph("select * from students where grade = ?;", placeholders);
```

## Handling Errors
Errors are handled in a very standard way across the entire library. Every method that has the potential for an error should have two versions: a "try" version and a "panic" version.

For instance, consider executing a non-returning query.

```rs
// Panic version.
let rows_affected = conn.execute_query("insert into students (name, grade) values ('jackson', 12);");

// Try version.
let rows_affected = match conn.try_execute_query("insert into students (name, grade) values ('jackson', 12);") {
    Err(err) => panic!(err.message()),
    Ok(val) => val,
}
```

It should be easy to see that every panic version of a method simply calls the try version and panics when an error is encountered.

The core library provides an error message and an error code for every library error encountered, whether from SQLx code or from DataJoint code. The error codes are primarily implemented for easy communication over the C FFI.

## Design Patterns
This section provides a general list of design patterns followed in the core library.

-  As much as possible, do not expose SQLx objects and traits over public methods. Wrapper objects and methods should be used as much as possible to have complete control over the DataJoint Core API. Thus, if SQLx changes, the DataJoint Core API can remain consistent.
- Propogate errors to callers using `try_*` methods.
- Do not use the `sqlx::any` types, for they are too restrictive for several use cases. Instead, many types are enums that wrap a specific SQLx type.
  - For instance, the `TableRow` object is actually an enum with a high-level API for working with internal values regardless of variant. Some complex scenarios, such as decoding unsigned integers, require the specific type of SQL database to be known. Enums should be unwrapped only when absolutely necessary, which is when MySQL and Postgres databases have different implementations.