use crate::results::table_row_vector::TableRowVector;
use datajoint_core::connection::{Cursor, Executor};
use datajoint_core::error::ErrorCode;
use datajoint_core::results::TableRow;
use libc::{c_char, c_void, free, malloc, size_t};
use std::ffi::CStr;

#[no_mangle]
pub unsafe extern "C" fn executor_new<'c>() -> *mut Executor<'c> {
    malloc(std::mem::size_of::<Executor<'c>> as size_t) as *mut Executor
}

#[no_mangle]
pub extern "C" fn executor_free(this: *mut Executor) {
    if this.is_null() {
        return;
    }
    unsafe {
        free(this as *mut c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn executor_execute(
    this: *mut Executor,
    query: *const c_char,
    out_size: *mut u64,
) -> i32 {
    if this.is_null() {
        return ErrorCode::NullNotAllowed as i32;
    }
    let executor = { &mut *this };
    let query_str = { CStr::from_ptr(query).to_string_lossy().to_owned() };
    match executor.try_execute(&query_str.to_string()) {
        Err(error) => error.code() as i32,
        Ok(value) => {
            *out_size = value;
            ErrorCode::Success as i32
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn executor_fetch_one(
    this: *mut Executor,
    query: *const c_char,
    out: *mut TableRow,
) -> i32 {
    if this.is_null() {
        return ErrorCode::NullNotAllowed as i32;
    }
    let executor = { &mut *this };
    let query_str = { CStr::from_ptr(query).to_string_lossy().to_owned() };
    match executor.try_fetch_one(&query_str.to_string()) {
        Err(error) => error.code() as i32,
        Ok(row) => {
            *out = row;
            ErrorCode::Success as i32
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn executor_fetch_all(
    this: *mut Executor,
    query: *const c_char,
    out: *mut TableRowVector,
) -> i32 {
    if this.is_null() {
        return ErrorCode::NullNotAllowed as i32;
    }
    let executor = { &mut *this };
    let query_str = { CStr::from_ptr(query).to_string_lossy().to_owned() };
    match executor.try_fetch_all(&query_str.to_string()) {
        Err(error) => error.code() as i32,
        Ok(rows) => {
            *out = TableRowVector::new(rows);
            ErrorCode::Success as i32
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn executor_cursor(this: *mut Executor, query: *const c_char) -> *mut Cursor {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let executor = { &*this };
    let query_str = match CStr::from_ptr(query).to_str() {
        Err(_) => return std::ptr::null_mut(),
        Ok(value) => value,
    };
    Box::into_raw(Box::new(executor.cursor(query_str)))
}
