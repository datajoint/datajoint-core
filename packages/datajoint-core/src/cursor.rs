use std::collections::HashMap;
use sqlx::Executor;
use sqlx::Row;


// Used to execute queries and access their results.
// Wraps sqlx::Executor.
pub struct Cursor {
    executor: &impl sqlx::Executor;
}
impl Cursor {
    fn new() -> Cursor {
        Cursor {

        }
    }

    // TODO: Needs to return Cursor
    fn load_executor(&self, executor: &impl sqlx::Executor) -> Cursor {
        self.executor = executor;
    }

    // TODO: are these good places to use return a Result?

    // Fetches all of the query results.
    fn fetch_all(&self) -> Result<TableRowVector, Error> {
        // fetch all query results and return a TableRowVector on success
        self.executor.fetch_all();
    }

    // Fetches the next result of the query.
    fn fetch_one(&self) -> Result<TableRow, Error> {
        self.executor.fetch_one();

    }
}

// Wraps a vector of database rows.
pub struct TableRowVector {
    table_rows: Vec<TableRow>;
}
impl TableRowVector {
    fn new() -> TableRowVector {
        TableRowVector {
            // data fields...
        } 
    }
    // Return number of rows in vector
    fn row_count(&self) -> usize {
        self.table_rows.len();
    }

    // Gets a result row by integer index.
    // Returns the row object.
    fn get(&self, index: usize) -> TableRow {
        self.table_rows[index];
    }

    // Load rows in...
    fn insert(&mut self, row: TableRow) {
        self.table_rows.push(row);
    }
}

// Wraps a dictionary-like object on the Rust side.
// Used to read a single row.
// Roughly equivalent to a row in SQLx.
pub struct TableRow {
    row: &impl sqlx::Row;
}
impl TableRow {
    fn new() -> TableRow {
        TableRow {
            
        }
    }

    //TODO: Needs to return Cursor 
    fn load_row(&self, row: &impl sqlx::Row) -> Cursor {
        self.row = row;
    }

    fn column_count(&self) -> usize {
        self.row.len();
    }

    // Gets column value by integer index.
    // Value is returned. Unknown what type!
    fn get(&self, ordinal: usize) -> T {
        self.row.get(ordinal);
    }

    // Gets column value by column name.
    // Value is returned. Unknown what type!
    fn get(&self, column_name: &str) -> T {
        self.row.get(column_name);
    }

    fn is_empty(&self) -> bool {
        self.row.is_empty();
    }
}
