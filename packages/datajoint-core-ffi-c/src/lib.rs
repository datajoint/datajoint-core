extern crate datajoint_core;

use datajoint_core::connection::Connection;
use datajoint_core::results::TableRow;
use libc::c_char;
use std::ffi::{CString, CStr};

#[no_mangle]
pub extern "C" fn connection_new(uri: *const c_char) -> *mut Connection {
    let uri = unsafe {
        assert!(!uri.is_null());
        CStr::from_ptr(uri)
    };

    let uri_str = uri.to_str().unwrap();

    Box::into_raw(Box::new(Connection::new(uri_str.to_string())))
}

#[no_mangle]
pub extern "C" fn connection_free(ptr: *mut Connection) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn connection_connect(ptr: *mut Connection) {
    let database = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    database.connect();
}

#[no_mangle]
pub extern "C" fn connection_raw_query(ptr: *const Connection, query: *const c_char) -> usize {
    let database: &Connection = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let query = unsafe {
        assert!(!query.is_null());
        CStr::from_ptr(query)
    };
    let query_str: &str = query.to_str().unwrap();
    database.raw_query(query_str).fetch_all().len()
}

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

macro_rules! build_table_row_get {
    ($func_name:ident, $data_type:ty) => {
        #[no_mangle] pub extern "C" fn $func_name(ptr: *const TableRow, column_name: *const c_char, mut _out: $data_type) -> i8 {
            let row: &TableRow = unsafe {
                if (ptr.is_null()) {
                    return -1;
                }
                &*ptr
            };

            let col_name = unsafe {
                if (column_name.is_null()) {
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
    }
}

build_table_row_get!(table_row_get_char_n, *mut c_char);
build_table_row_get!(table_row_get_date, *mut c_char);
