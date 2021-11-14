use crate::error::datajoint_core_set_last_error;
use crate::results::TableRowVector;
use crate::util;
use datajoint_core::results::TableRow;
use datajoint_core::{
    connection::Cursor,
    error::{DataJointError, ErrorCode},
};

/// Frees a cursor.
#[no_mangle]
pub unsafe extern "C" fn cursor_free(this: *mut Cursor) {
    if !this.is_null() {
        Box::from_raw(this);
    }
}

/// Fetches the next row.
#[no_mangle]
pub unsafe extern "C" fn cursor_next(this: *mut Cursor, out: *mut *mut TableRow) -> i32 {
    if this.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }
    let cursor = &mut *this;
    match cursor.try_next() {
        Err(error) => datajoint_core_set_last_error(error) as i32,
        Ok(value) => {
            util::mem::handle_output_ptr(out, value);
            ErrorCode::Success as i32
        }
    }
}

/// Fetches all remaining rows.
#[no_mangle]
pub unsafe extern "C" fn cursor_rest(this: *mut Cursor, out: *mut *mut TableRowVector) -> i32 {
    if this.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }
    let cursor = &mut *this;
    match cursor.try_rest() {
        Err(error) => datajoint_core_set_last_error(error) as i32,
        Ok(value) => {
            util::mem::handle_output_ptr(out, TableRowVector::new(value));
            ErrorCode::Success as i32
        }
    }
}
