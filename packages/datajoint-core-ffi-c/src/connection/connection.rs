use crate::error::datajoint_core_set_last_error;
use crate::util;
use datajoint_core::connection::{Connection, ConnectionSettings, Cursor, Executor};
use datajoint_core::error::{DataJointError, ErrorCode};
use datajoint_core::placeholders::PlaceholderArgumentVector;
use libc::c_char;
use std::ffi::CStr;

/// Allocates a new connection.
///
/// The new connection instance takes ownership of the settings object passed in.
/// The settings object will be deallocated when the settings object is deallocated.
/// Library users should not manually free a [`ConnectionSettings`] object after it
/// is passed into this function.
#[no_mangle]
pub unsafe extern "C" fn connection_new(settings: *mut ConnectionSettings) -> *mut Connection {
    if settings.is_null() {
        return std::ptr::null_mut();
    }

    Box::into_raw(Box::new(Connection::new(*Box::from_raw(settings))))
}

/// Frees a connection.
#[no_mangle]
pub unsafe extern "C" fn connection_free(this: *mut Connection) {
    if !this.is_null() {
        Box::from_raw(this);
    }
}

/// Checks if the connection is still connected.
#[no_mangle]
pub extern "C" fn connection_is_connected(this: *mut Connection) -> i32 {
    if this.is_null() {
        false as i32
    } else {
        let connection = unsafe { &*this };
        connection.is_connected() as i32
    }
}

/// Starts the connection to the SQL database according to the settings the connection
/// was initialized with.
#[no_mangle]
pub extern "C" fn connection_connect(this: *mut Connection) -> i32 {
    if this.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }
    let connection = unsafe { &mut *this };
    match connection.connect() {
        Err(error) => datajoint_core_set_last_error(error) as i32,
        Ok(_) => ErrorCode::Success as i32,
    }
}

/// Disconnects from the SQL database.
///
/// If the database connection has already been disconnected, this method
/// is a no-op.
///
/// The connection can be restarted if desired.
#[no_mangle]
pub extern "C" fn connection_disconnect(this: *mut Connection) -> i32 {
    if this.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }
    let connection = unsafe { &mut *this };
    connection.disconnect();
    ErrorCode::Success as i32
}

/// Restarts the connection to the SQL database according to the internal settings object.
#[no_mangle]
pub extern "C" fn connection_reconnect(this: *mut Connection) -> i32 {
    if this.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }
    let connection = unsafe { &mut *this };
    connection.disconnect();
    match connection.connect() {
        Err(error) => datajoint_core_set_last_error(error) as i32,
        Ok(_) => ErrorCode::Success as i32,
    }
}

/// Gets the pointer to the connection's internal settings object..
///
/// This pointer should not be freed.
#[no_mangle]
pub extern "C" fn connection_get_settings(this: *mut Connection) -> *mut ConnectionSettings {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let connection = unsafe { &mut *this };
    &connection.settings as *const ConnectionSettings as *mut ConnectionSettings
}

/// Creates an executor to interact with the database over this connection.
#[no_mangle]
pub unsafe extern "C" fn connection_executor(
    this: *mut Connection,
    out: *mut *mut Executor,
) -> i32 {
    if this.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }
    let connection = &mut *this;
    match connection.try_executor() {
        Err(error) => datajoint_core_set_last_error(error) as i32,
        Ok(executor) => {
            util::mem::handle_output_ptr(out, executor);
            ErrorCode::Success as i32
        }
    }
}

/// Executes the given non-returning query, returning the number of rows affected.
///
/// The third parameter can be `NULL` or a collection of placeholder arguments to
/// bind to the query. Once the query is executed, the [`PlaceholderArgumentVector`]
/// is owned and deallocated by the library. In other words, the caller does not
/// need to manually free the placeholder arguments after they are bound to a query.
#[no_mangle]
pub unsafe extern "C" fn connection_execute_query(
    this: *mut Connection,
    query: *const c_char,
    args: *mut PlaceholderArgumentVector,
    out: *mut u64,
) -> i32 {
    if this.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }
    let connection = &mut *this;
    let query_str = match CStr::from_ptr(query).to_str() {
        Err(_) => {
            return datajoint_core_set_last_error(DataJointError::new(ErrorCode::InvalidUtf8String))
                as i32
        }
        Ok(value) => value,
    };
    match if args.is_null() {
        connection.try_execute_query(query_str)
    } else {
        connection.try_execute_query_ph(query_str, *Box::from_raw(args))
    } {
        Err(error) => datajoint_core_set_last_error(error) as i32,
        Ok(value) => {
            if !out.is_null() {
                *out = value;
            }
            ErrorCode::Success as i32
        }
    }
}

/// Creates a cursor for iterating over the results of the given returning query.
///
/// The third parameter can be `NULL` or a collection of placeholder arguments to
/// bind to the query. Once the query is executed, the [`PlaceholderArgumentVector`]
/// is owned and deallocated by the library. In other words, the caller does not
/// need to manually free the placeholder arguments after they are bound to a query.
#[no_mangle]
pub unsafe extern "C" fn connection_fetch_query(
    this: *mut Connection,
    query: *const c_char,
    args: *mut PlaceholderArgumentVector,
    out: *mut *mut Cursor,
) -> i32 {
    if this.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }
    let connection = &mut *this;
    let query_str = match CStr::from_ptr(query).to_str() {
        Err(_) => {
            return datajoint_core_set_last_error(DataJointError::new(ErrorCode::InvalidUtf8String))
                as i32
        }
        Ok(value) => value,
    };
    match if args.is_null() {
        connection.try_fetch_query(query_str)
    } else {
        connection.try_fetch_query_ph(query_str, *Box::from_raw(args))
    } {
        Err(error) => datajoint_core_set_last_error(error) as i32,
        Ok(cursor) => {
            util::mem::handle_output_ptr(out, cursor);
            ErrorCode::Success as i32
        }
    }
}
