use datajoint_core::error::{DataJointError, Error, ErrorCode};
use datajoint_core::results::TableRow;
use libc::size_t;
use std::ptr;

/// A vector of table rows, which is used to communicate the results of returning
/// queries that return more than one row at a time.
pub struct TableRowVector {
    rows: Vec<TableRow>,
}

#[allow(dead_code)]
impl TableRowVector {
    /// Creates a new table row vector.
    pub fn new(table_rows: Vec<TableRow>) -> Self {
        return TableRowVector { rows: table_rows };
    }

    /// Returns the number of rows.
    pub fn row_count(&self) -> size_t {
        self.rows.len()
    }

    /// Returns a reference to a TableRow at the given index.
    pub fn get(&self, index: size_t) -> &TableRow {
        self.try_get(index).unwrap()
    }

    /// Returns a reference to a TableRow at the given index.
    pub fn try_get(&self, index: size_t) -> Result<&TableRow, Error> {
        let result = self.rows.get(index);
        match result {
            Some(value) => Ok(value),
            None => Err(DataJointError::new(ErrorCode::RowIndexOutOfBounds)),
        }
    }

    /// Inserts a TableRow into the vector.
    pub fn insert(&mut self, row: TableRow) {
        self.rows.push(row);
    }
}

/// Frees a table row vector, including all table rows inside.
#[no_mangle]
pub unsafe extern "C" fn table_row_vector_free(this: *mut TableRowVector) {
    if !this.is_null() {
        Box::from_raw(this);
    }
}

/// Gives the number of rows.
#[no_mangle]
pub extern "C" fn table_row_vector_size(this: *const TableRowVector) -> size_t {
    if this.is_null() {
        return 0;
    }
    let table_rows: &TableRowVector = unsafe { &*this };
    table_rows.row_count()
}

/// Gives an internal pointer to a `TableRow` at the given index.
///
/// This pointer should not be freed by the user. Instead, call `table_row_vector_free`
/// to free an entire table row vector.
#[no_mangle]
pub extern "C" fn table_row_vector_get(
    this: *const TableRowVector,
    index: size_t,
) -> *const TableRow {
    if this.is_null() {
        return ptr::null();
    }

    let table_rows: &TableRowVector = unsafe { &*this };
    table_rows.get(index)
}
