
extern crate datajoint_core;

use datajoint_core::connection::Connection;
use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn connection_new(
    host: *const c_char,
    user: *const c_char,
    password: *const c_char,
    reset: bool,
    use_tls: bool) -> *mut Connection {

    let host = unsafe {
        assert!(!host.is_null());
        CStr::from_ptr(host)
    };

    let host_str = host.to_str().unwrap();

    let user = unsafe {
        assert!(!user.is_null());
        CStr::from_ptr(user)
    };

    let user_str = user.to_str().unwrap();

    let password = unsafe {
        assert!(!password.is_null());
        CStr::from_ptr(password)
    };

    let password_str = password.to_str().unwrap();

    Box::into_raw(Box::new(Connection::new(host_str, user_str, password_str, reset, use_tls)))
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
pub extern "C" fn connection_query(
    ptr: *const Connection,
    query: *const c_char,
) -> u32 {
    let database = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let query = unsafe {
        assert!(!query.is_null());
        CStr::from_ptr(query)
    };
    let query_str = query.to_str().unwrap();
    database.raw_query(query_str)
}