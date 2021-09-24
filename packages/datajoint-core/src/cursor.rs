use std::collections::HashMap;


// Used to execute queries and access their results.
pub struct Cursor {
    // put whatever fields
    results: TableRowVector;
    row_number: usize;
    row_count: usize;
}
impl Cursor {
    fn new() -> Cursor {
        Cursor {
            // whatever data fields it...
            row_number = 0;
            row_count = 0;
        }
    }

    // Fetches all of the query results.
    // Returns an integer code. 0 on success, nonzero represents an error code.
    fn fetch_all(&self) -> TableRowVector {
        // results
    }

    // Fetches the next result of the query.
    // Returns an integer code. 0 on success, nonzero represents an error code.
    fn fetch_one(&self) -> TableRow {
        // results.get(row_number);

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
    data: HashMap<T, T>;
}
impl TableRow {
    fn new() -> TableRow {
        TableRow {
            
        }
    }

    fn column_count(&self) -> usize {
        self.data.len()
    }
    // Gets column value by integer index.
    // Value is returned. Unknown what type!
    fn get(&self, index: usize) -> T {

    }

    // Gets column value by column name.
    // Value is returned. Unknown what type!
    fn get(&self, column_name: &str) -> T {
        self.data.get(column_name)
    }

    fn load_row(&self, row: sqlx::Row) -> Cursor {

    }

    fn is_empty(&self) -> bool {
        self.data.is_empty();
    }
}
