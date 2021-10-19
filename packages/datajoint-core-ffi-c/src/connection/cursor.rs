use crate::results::table_row_vector::TableRowVector;
use datajoint_core::results::TableRow;
use datajoint_core::{connection::Cursor, error::ErrorCode};
use libc::{c_void, free, malloc, size_t};

#[no_mangle]
pub unsafe extern "C" fn cursor_new<'c>() -> *mut Cursor<'c> {
    malloc(std::mem::size_of::<Cursor<'c>> as size_t) as *mut Cursor
}

#[no_mangle]
pub extern "C" fn cursor_free(this: *mut Cursor) {
    if this.is_null() {
        return;
    }
    unsafe {
        free(this as *mut c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn cursor_next(this: *mut Cursor, out: *mut TableRow) -> i32 {
    if this.is_null() {
        return ErrorCode::NullNotAllowed as i32;
    }
    let cursor = &mut *this;
    match cursor.try_next() {
        Err(error) => error.code() as i32,
        Ok(value) => {
            *out = value;
            ErrorCode::Success as i32
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cursor_rest(this: *mut Cursor, out: *mut TableRowVector) -> i32 {
    if this.is_null() {
        return ErrorCode::NullNotAllowed as i32;
    }
    let cursor = &mut *this;
    match cursor.try_rest() {
        Err(error) => error.code() as i32,
        Ok(value) => {
            *out = TableRowVector { rows: value };
            ErrorCode::Success as i32
        }
    }
}
