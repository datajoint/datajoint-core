use crate::util;
use datajoint_core::connection::{Connection, ConnectionSettings, Cursor, Executor};
use datajoint_core::error::ErrorCode;
use datajoint_core::placeholders::PlaceholderArgumentVector;
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
pub unsafe extern "C" fn connection_free(this: *mut Connection) {
    if !this.is_null() {
        Box::from_raw(this);
    }
}

#[no_mangle]
pub extern "C" fn connection_is_connected(this: *mut Connection) -> i32 {
    if this.is_null() {
        false as i32
    } else {
        let connection = unsafe { &*this };
        connection.is_connected() as i32
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
    connection.disconnect();
    ErrorCode::Success as i32
}

#[no_mangle]
pub extern "C" fn connection_reconnect(this: *mut Connection) -> i32 {
    if this.is_null() {
        return ErrorCode::NullNotAllowed as i32;
    }
    let connection = unsafe { &mut *this };
    connection.disconnect();
    match connection.connect() {
        Err(error) => error.code() as i32,
        Ok(_) => ErrorCode::Success as i32,
    }
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
pub unsafe extern "C" fn connection_executor(
    this: *mut Connection,
    out: *mut *mut Executor,
) -> i32 {
    if this.is_null() {
        return ErrorCode::NullNotAllowed as i32;
    }
    let connection = &mut *this;
    match connection.try_executor() {
        Err(error) => error.code() as i32,
        Ok(executor) => {
            util::mem::handle_output_ptr(out, executor);
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
    let connection = &mut *this;
    let query_str = match CStr::from_ptr(query).to_str() {
        Err(_) => return ErrorCode::InvalidCString as i32,
        Ok(value) => value,
    };
    match connection.try_execute_query(query_str) {
        Err(error) => {
            println!("{}", error.message());
            error.code() as i32
        }
        Ok(value) => {
            if !out.is_null() {
                *out = value;
            }
            ErrorCode::Success as i32
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn connection_execute_query_ph(
    this: *mut Connection,
    query: *const c_char,
    ph_args: *mut PlaceholderArgumentVector,
    out: *mut u64,
) -> i32 {
    if this.is_null() || query.is_null() || ph_args.is_null() {
        return ErrorCode::NullNotAllowed as i32;
    }
    let connection = { &mut *this };
    let args = Box::from_raw(ph_args);
    let query_str = match CStr::from_ptr(query).to_str() {
        Err(_) => return ErrorCode::InvalidCString as i32,
        Ok(value) => value,
    };
    match connection.try_execute_query_ph(query_str, *args) {
        Err(error) => error.code() as i32,
        Ok(value) => {
            if out.is_null() {
                *out = value;
            }
            ErrorCode::Success as i32
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn connection_fetch_query(
    this: *mut Connection,
    query: *const c_char,
    out: *mut *mut Cursor,
) -> i32 {
    if this.is_null() {
        return ErrorCode::NullNotAllowed as i32;
    }
    let connection = &mut *this;
    let query_str = match CStr::from_ptr(query).to_str() {
        Err(_) => return ErrorCode::InvalidCString as i32,
        Ok(value) => value,
    };
    match connection.try_fetch_query(query_str) {
        Err(error) => error.code() as i32,
        Ok(cursor) => {
            util::mem::handle_output_ptr(out, cursor);
            ErrorCode::Success as i32
        }
    }
}
