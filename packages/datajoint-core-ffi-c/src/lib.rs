use std::ffi::CStr;
use std::os::raw::c_char;
use datajoint_core::connection::Connection;

extern crate libc;

#[no_mangle]
pub extern "C" fn connection_new() -> *mut Connection {
    Box::into_raw(Box::new(Connection::new()))
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
pub extern "C" fn connection_connect(
    ptr: *mut Connection,
) {

    let database = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    // let host = unsafe {
    //     assert!(!host.is_null());
    //     CStr::from_ptr(host)
    // };
    // let host_str = host.to_str().unwrap();
    //
    // let user = unsafe {
    //     assert!(!user.is_null());
    //     CStr::from_ptr(user)
    // };
    // let user_str = user.to_str().unwrap();
    //
    // let password = unsafe {
    //     assert!(!password.is_null());
    //     CStr::from_ptr(password)
    // };
    // let password_str = password.to_str().unwrap();


    database.connect();
    // should this be returning something ???
}

#[no_mangle]
pub extern "C" fn connection_query(
    ptr: *mut Connection,
    query: *const c_char
) {
    let database = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    let query = unsafe {
        assert!(!query.is_null());
        CStr::from_ptr(query)
    };

    let query_str = query.to_str().unwrap();
    database.query(query_str)
}

