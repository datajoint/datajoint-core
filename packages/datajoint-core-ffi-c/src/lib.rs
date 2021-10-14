mod placeholders;

extern crate datajoint_core;

use datajoint_core::connection::Connection;
use libc::c_char;
use std::ffi::CStr;

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
