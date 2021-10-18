use datajoint_core::connection::{Connection, ConnectionSettings, Cursor, Executor};
use datajoint_core::error::ErrorCode;
use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn connection_new(this: *mut ConnectionSettings) -> *mut Connection {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let settings = unsafe { Box::from_raw(this) };

    Box::into_raw(Box::new(Connection::new(*settings)))
}

#[no_mangle]
pub extern "C" fn connection_free(this: *mut Connection) {
    if this.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(this);
    }
}

#[no_mangle]
pub extern "C" fn connection_connect(this: *mut Connection) -> i32 {
    if this.is_null() {
        return ErrorCode::NullNotAllowed as i32;
    }
    let connection = unsafe { &mut *this };
    match connection.connect() {
        Err(error) => error.code() as i32,
        Ok(_) => ErrorCode::Success as i32,
    }
}

#[no_mangle]
pub extern "C" fn connection_disconnect(this: *mut Connection) -> i32 {
    if this.is_null() {
        return ErrorCode::NullNotAllowed as i32;
    }
    let connection = unsafe { &mut *this };
    match connection.disconnect() {
        Err(_) => 1, // TODO: return error.code when disconnect is fixed.
        Ok(_) => ErrorCode::Success as i32,
    }
}

#[no_mangle]
pub extern "C" fn connection_reconnect(this: *mut Connection) -> i32 {
    if this.is_null() {
        return ErrorCode::NullNotAllowed as i32;
    }
    let connection = unsafe { &mut *this };
    match connection.disconnect() {
        Err(_) => return ErrorCode::NotConnected as i32,
        Ok(_) => (),
    };
    match connection.connect() {
        Err(error) => return error.code() as i32,
        Ok(_) => (),
    };
    ErrorCode::Success as i32
}

#[no_mangle]
pub extern "C" fn connection_get_settings(this: *mut Connection) -> *mut ConnectionSettings {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let connection = unsafe { &mut *this };
    &connection.settings as *const ConnectionSettings as *mut ConnectionSettings
}

#[no_mangle]
pub unsafe extern "C" fn connection_executor(this: *mut Connection, out: *mut Executor) -> i32 {
    if this.is_null() {
        return ErrorCode::NullNotAllowed as i32;
    }
    let connection = &mut *this;
    match connection.try_executor() {
        Err(error) => error.code() as i32,
        Ok(executor) => {
            *out = executor;
            ErrorCode::Success as i32
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn connection_execute_query(
    this: *mut Connection,
    query: *const c_char,
    out: *mut u64,
) -> i32 {
    if this.is_null() {
        return ErrorCode::NullNotAllowed as i32;
    }
    let connection = { &mut *this };
    let query_str = match CStr::from_ptr(query).to_str() {
        Err(_) => return ErrorCode::InvalidCString as i32,
        Ok(value) => value
    };
    match connection.try_execute_query(query_str) {
        Err(error) => error.code() as i32,
        Ok(value) => {
            *out = value;
            ErrorCode::Success as i32
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn connection_fetch_query(
    this: *mut Connection,
    query: *const c_char,
    out_cursor: *mut Cursor,
) -> i32 {
    if this.is_null() {
        return ErrorCode::NullNotAllowed as i32;
    }
    let connection = &mut *this;
    let query_str = match CStr::from_ptr(query).to_str() {
        Err(_) => return ErrorCode::InvalidCString as i32,
        Ok(value) => value
    };
    match connection.try_fetch_query(query_str) {
        Err(error) => error.code() as i32,
        Ok(cursor) => {
            *out_cursor = cursor;
            ErrorCode::Success as i32
        }
    }
}
