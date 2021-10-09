extern crate datajoint_core;

use datajoint_core::results::TableRow;
use libc::c_char;
use std::ffi::{CString, CStr};

/// TableRow C FFI
#[no_mangle]
pub extern "C" fn table_row_free(ptr: *mut TableRow) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn table_row_column_count(ptr: *const TableRow) -> usize {
    let row: &TableRow = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    row.column_count()
}

#[no_mangle] 
pub extern "C" fn table_row_get_char_n(ptr: *const TableRow, column_name: *const c_char, mut _out: *mut c_char) -> i8 {
    let row: &TableRow = unsafe {
        if ptr.is_null() {
            return -1;
        }
        &*ptr
    };

    let col_name = unsafe {
        if column_name.is_null() {
            return -1;
        }
        CStr::from_ptr(column_name)
    };

    let col_name = col_name.to_str();
    let col_name : &str = match col_name {
        Ok(name) => name,
        Err(_) => return -1
    };

    let res_str: &str = row.get(col_name);
    _out = CString::new(res_str).unwrap().into_raw();
    0
}

#[no_mangle] 
pub extern "C" fn table_row_get_date(ptr: *const TableRow, column_name: *const c_char, mut _out: *mut c_char) -> i8 {
    let row: &TableRow = unsafe {
        if ptr.is_null() {
            return -1;
        }
        &*ptr
    };

    let col_name = unsafe {
        if column_name.is_null() {
            return -1;
        }
        CStr::from_ptr(column_name)
    };

    let col_name = col_name.to_str();
    let col_name : &str = match col_name {
        Ok(name) => name,
        Err(_) => return -1
    };

    let res_str: &str = row.get(col_name);
    _out = CString::new(res_str).unwrap().into_raw();
    0
}

#[no_mangle] 
pub extern "C" fn table_row_get_tinyint(ptr: *const TableRow, column_name: *const c_char, mut _out: *mut c_char) -> i8 {
    let row: &TableRow = unsafe {
        if ptr.is_null() {
            return -1;
        }
        &*ptr
    };

    let col_name = unsafe {
        if column_name.is_null() {
            return -1;
        }
        CStr::from_ptr(column_name)
    };

    let col_name = col_name.to_str();
    let col_name : &str = match col_name {
        Ok(name) => name,
        Err(_) => return -1
    };

    let data: &str = row.get(col_name);
    _out = CString::new(data).unwrap().into_raw();
    0
}

#[no_mangle] 
pub extern "C" fn table_row_get_tinyint_unsigned(ptr: *const TableRow, column_name: *const c_char, mut _out: *mut c_char) -> i8 {
    let row: &TableRow = unsafe {
        if ptr.is_null() {
            return -1;
        }
        &*ptr
    };

    let col_name = unsafe {
        if column_name.is_null() {
            return -1;
        }
        CStr::from_ptr(column_name)
    };

    let col_name = col_name.to_str();
    let col_name : &str = match col_name {
        Ok(name) => name,
        Err(_) => return -1
    };

    let data: &str  = row.get(col_name);
    _out = CString::new(data).unwrap().into_raw();
    0
}

#[no_mangle] 
pub extern "C" fn table_row_get_ordinal_char_n(ptr: *const TableRow, column: usize , mut _out: *mut c_char) -> i8 {
    let row: &TableRow = unsafe {
        if ptr.is_null() {
            return -1;
        }
        &*ptr
    };

    let data : &str = row.get(column);
    _out = CString::new(data).unwrap().into_raw();
    0
}

#[no_mangle] 
pub extern "C" fn table_row_get_ordinal_date(ptr: *const TableRow, column: usize , mut _out: *mut c_char) -> i8 {
    let row: &TableRow = unsafe {
        if ptr.is_null() {
            return -1;
        }
        &*ptr
    };

    let data: &str = row.get(column);
    _out = CString::new(data).unwrap().into_raw();
    0
}

#[no_mangle] 
pub extern "C" fn table_row_get_ordinal_tinyint(ptr: *const TableRow, column: usize , mut _out: *mut c_char) -> i8 {
    let row: &TableRow = unsafe {
        if ptr.is_null() {
            return -1;
        }
        &*ptr
    };

    let data: &str = row.get(column);
    _out = CString::new(data).unwrap().into_raw();
    0
}

#[no_mangle] 
pub extern "C" fn table_row_get_ordinal_tinyint_unsigned(ptr: *const TableRow, column: usize , mut _out: *mut c_char) -> i8 {
    let row: &TableRow = unsafe {
        if ptr.is_null() {
            return -1;
        }
        &*ptr
    };

    let data: &str = row.get(column);
    _out = CString::new(data).unwrap().into_raw();
    0
}