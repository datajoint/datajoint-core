extern crate datajoint_core;

use datajoint_core::results::TableRow;
use std::ptr;

/// Creates a vector of TableRows
pub struct TableRowVector {
    rows: Vec<TableRow>
}
impl TableRowVector {
    /// Creates a new table row vector
    pub fn new(table_rows: Vec<TableRow>) -> Self {
        return TableRowVector { 
            rows: table_rows
        };
    }

    pub fn new_cffi() -> Self {
        return TableRowVector { 
            rows: Vec::new()
        };
    }

    /// Returns the number of rows
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Returns a reference to a TableRow at the given index
    /// 
    /// Panics on error
    pub fn get(&self, index: usize) -> &TableRow {
        self.try_get(index).unwrap()
    }

    /// Returns a reference to a TableRow at the given index
    pub fn try_get(&self, index: usize) -> Result<&TableRow, &str> {
        let result = self.rows.get(index);
        match result {
            Some(value) => Ok(value),
            None => Err("Index out of range"),
        }
    }

    /// Inserts a TableRow into the vector
    pub fn insert(&mut self, row: TableRow) {
        self.rows.push(row);
    }
}

#[no_mangle]
pub extern "C" fn table_row_vector_new() ->  *mut TableRowVector {
    Box::into_raw(Box::new(TableRowVector::new_cffi()))
}

#[no_mangle]
pub extern "C" fn table_row_vector_free(ptr: *mut TableRowVector) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn table_row_vector_row_count(ptr: *const TableRowVector) -> usize {
    let table_rows: &TableRowVector = unsafe {
        if ptr.is_null() {
            return 0;
        }
        &*ptr
    };
    table_rows.row_count()
}

#[no_mangle]
pub extern "C" fn table_row_vector_get(ptr: *const TableRowVector, index: usize) -> *const TableRow  {
    let table_rows: &TableRowVector = unsafe {
        if ptr.is_null() {
            return ptr::null();
        }
        &*ptr
    };
    table_rows.get(index)
}
