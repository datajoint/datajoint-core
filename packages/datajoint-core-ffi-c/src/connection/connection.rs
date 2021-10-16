use datajoint_core::connection::{Connection, ConnectionSettings, Cursor, Executor};
use datajoint_core::error::ErrorCode;
use std::ffi::CStr;
use libc::c_char;

#[no_mangle]
pub extern "C" fn connection_new(ptr: *mut ConnectionSettings) -> *mut Connection {
    if ptr.is_null() {
        return std::ptr::null_mut();
    }
    let settings = unsafe {
        Box::from_raw(ptr)
    };

    Box::into_raw(Box::new(Connection::new(*settings)))
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
pub extern "C" fn connection_connect(ptr: *mut Connection) -> i32 {
    if ptr.is_null() {
        return 0;
    }
    let database = unsafe {
        &mut *ptr
    };
    match database.connect() {
        Err(error) => error.code() as i32,
        Ok(_) => ErrorCode::Success as i32,
    }
}

#[no_mangle]
pub extern "C" fn connection_disconnect(ptr: *mut Connection) -> i32 {
    if ptr.is_null() {
        return 0;
    }
    let database = unsafe {
        &mut *ptr
    };
    match database.disconnect() {
        Err(_) => 1, // TODO: return error.code when disconnect is fixed.
        Ok(_) => ErrorCode::Success as i32,
    }
}

#[no_mangle]
pub extern "C" fn connection_reconnect(ptr: *mut Connection) -> i32 {
    if ptr.is_null() {
        return ErrorCode::NullNotAllowed as i32;
    }
    let database = unsafe {
        &mut *ptr
    };
    match database.disconnect() {
        Err(_) => return ErrorCode::NotConnected as i32,
        Ok(_) => ()
    };
    match database.connect() {
        Err(error) => return error.code() as i32,
        Ok(_) => ()
    };
    ErrorCode::Success as i32
}

#[no_mangle]
pub extern "C" fn connection_get_settings(ptr: *mut Connection) -> *mut ConnectionSettings {
    if ptr.is_null() {
        return std::ptr::null_mut();
    }
    let database = unsafe {
        &mut *ptr
    };
    &database.settings as *const ConnectionSettings as *mut ConnectionSettings
}

#[no_mangle]
pub extern "C" fn connection_executor(ptr: *mut Connection, out: *mut Executor) -> i32 {
    if ptr.is_null() {
        return ErrorCode::NullNotAllowed as i32;
    }
    let database = unsafe {
        &mut *ptr
    };


    ErrorCode::Success as i32
}

#[no_mangle]
pub extern "C" fn connection_execute_query(ptr: *mut Connection, query: *const c_char, out: usize) -> i32 {
    if ptr.is_null() {
        return ErrorCode::NullNotAllowed as i32;
    }
    let database = unsafe {
        &mut *ptr
    };
    let query_str = unsafe {
        CStr::from_ptr(query).to_string_lossy().to_owned()
    };

    match database.try_execute_query(&query_str.to_string()) {
        Err(error) => error.code() as i32,
        Ok(_) => ErrorCode::Success as i32
    }

}

#[no_mangle]
pub extern "C" fn connection_fetch_query(ptr: *mut Connection, query: *const c_char, out_cursor: *mut Cursor) -> i32 {
    if ptr.is_null() {
        return ErrorCode::NullNotAllowed as i32;
    }
    let database = unsafe {
        &mut *ptr
    };
    let query_str = unsafe {
        CStr::from_ptr(query).to_string_lossy().to_owned()
    };



    ErrorCode::Success as i32
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
    // database.raw_query(query_str).fetch_all().len()
    1
}
