use crate::error::datajoint_core_set_last_error;
use crate::results::table_row_vector::TableRowVector;
use crate::util;
use datajoint_core::connection::{Cursor, Executor};
use datajoint_core::error::{DataJointError, ErrorCode};
use datajoint_core::placeholders::PlaceholderArgumentVector;
use datajoint_core::results::TableRow;
use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
pub unsafe extern "C" fn executor_free(this: *mut Executor) {
    if !this.is_null() {
        Box::from_raw(this);
    }
}

#[no_mangle]
pub unsafe extern "C" fn executor_execute(
    this: *mut Executor,
    query: *const c_char,
    out_size: *mut u64,
) -> i32 {
    if this.is_null() || query.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }
    let executor = &*this;
    let query_str = match CStr::from_ptr(query).to_str() {
        Err(_) => {
            return datajoint_core_set_last_error(DataJointError::new(ErrorCode::InvalidUtf8String))
                as i32
        }
        Ok(value) => value,
    };
    match executor.try_execute(&query_str.to_string()) {
        Err(error) => datajoint_core_set_last_error(error) as i32,
        Ok(value) => {
            if !out_size.is_null() {
                *out_size = value;
            }
            ErrorCode::Success as i32
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn executor_fetch_one(
    this: *mut Executor,
    query: *const c_char,
    out: *mut *mut TableRow,
) -> i32 {
    if this.is_null() || query.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }
    let executor = &*this;
    let query_str = match CStr::from_ptr(query).to_str() {
        Err(_) => {
            return datajoint_core_set_last_error(DataJointError::new(ErrorCode::InvalidUtf8String))
                as i32
        }
        Ok(value) => value,
    };
    match executor.try_fetch_one(&query_str.to_string()) {
        Err(error) => datajoint_core_set_last_error(error) as i32,
        Ok(row) => {
            util::mem::handle_output_ptr(out, row);
            ErrorCode::Success as i32
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn executor_fetch_all(
    this: *mut Executor,
    query: *const c_char,
    out: *mut *mut TableRowVector,
) -> i32 {
    if this.is_null() || query.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }
    let executor = &*this;
    let query_str = match CStr::from_ptr(query).to_str() {
        Err(_) => {
            return datajoint_core_set_last_error(DataJointError::new(ErrorCode::InvalidUtf8String))
                as i32
        }
        Ok(value) => value,
    };
    match executor.try_fetch_all(&query_str.to_string()) {
        Err(error) => datajoint_core_set_last_error(error) as i32,
        Ok(rows) => {
            util::mem::handle_output_ptr(out, TableRowVector::new(rows));
            ErrorCode::Success as i32
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn executor_cursor<'c>(
    this: *mut Executor<'c>,
    query: *const c_char,
    args: *mut PlaceholderArgumentVector,
    out: *mut *mut Cursor<'c>,
) -> i32 {
    if this.is_null() || query.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }
    let executor = &*this;
    let query_str = match CStr::from_ptr(query).to_str() {
        Err(_) => {
            return datajoint_core_set_last_error(DataJointError::new(ErrorCode::InvalidUtf8String))
                as i32
        }
        Ok(value) => value,
    };
    match if args.is_null() {
        executor.cursor(query_str)
    } else {
        executor.cursor_ph(query_str, *Box::from_raw(args))
    } {
        Err(error) => datajoint_core_set_last_error(error) as i32,
        Ok(cursor) => {
            util::mem::handle_output_ptr(out, cursor);
            ErrorCode::Success as i32
        }
    }
}
