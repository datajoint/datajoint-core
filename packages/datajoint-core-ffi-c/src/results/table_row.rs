use crate::error::datajoint_core_set_last_error;
use crate::util;
use datajoint_core::error::{DataJointError, ErrorCode};
use datajoint_core::results::TableColumnRef;
use datajoint_core::results::TableRow;
use libc::{c_char, size_t};
use std::ffi::CStr;

/// Frees an instance of TableRow.
#[no_mangle]
pub unsafe extern "C" fn table_row_free(this: *mut TableRow) {
    if !this.is_null() {
        Box::from_raw(this);
    }
}

/// Returns if the row is empty.
#[no_mangle]
pub unsafe extern "C" fn table_row_is_empty(this: *const TableRow) -> i32 {
    if this.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }
    (&*this).is_empty() as i32
}

/// Utility method for returning the number of columns in the row
/// without constructing an intermediate vector.
#[no_mangle]
pub unsafe extern "C" fn table_row_column_count(this: *const TableRow) -> size_t {
    if this.is_null() {
        return 0;
    }
    (&*this).column_count()
}

#[no_mangle]
pub unsafe extern "C" fn table_row_columns(
    this: *const TableRow,
    out_columns: *mut *mut TableColumnRef,
    columns_size: *mut size_t,
) -> i32 {
    if this.is_null() || out_columns.is_null() || columns_size.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }
    let mut cols = std::mem::ManuallyDrop::new((&*this).columns());
    cols.shrink_to_fit();
    *columns_size = cols.len();
    *out_columns = cols.as_mut_ptr();
    ErrorCode::Success as i32
}

#[no_mangle]
pub unsafe extern "C" fn table_row_columns_advance(
    columns: *mut TableColumnRef,
    index: size_t,
) -> *mut TableColumnRef {
    if columns.is_null() {
        std::ptr::null_mut()
    } else {
        columns.add(index)
    }
}

#[no_mangle]
pub unsafe extern "C" fn table_row_columns_free(
    out_columns: *mut TableColumnRef,
    columns_size: size_t,
) {
    if !out_columns.is_null() {
        Vec::from_raw_parts(out_columns, columns_size, columns_size);
    }
}

#[no_mangle]
pub unsafe extern "C" fn table_row_get_column_with_name(
    this: *const TableRow,
    column_name: *const c_char,
    out: *mut *mut TableColumnRef,
) -> i32 {
    if this.is_null() || column_name.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }

    let column_name = match CStr::from_ptr(column_name).to_str() {
        Err(_) => {
            return datajoint_core_set_last_error(DataJointError::new(ErrorCode::InvalidUtf8String))
                as i32
        }
        Ok(string) => string,
    };

    let col_ref: TableColumnRef = match (&*this).try_column(column_name) {
        Err(err) => return datajoint_core_set_last_error(err) as i32,
        Ok(value) => value,
    };
    util::mem::handle_output_ptr(out, col_ref);
    ErrorCode::Success as i32
}

#[no_mangle]
pub unsafe extern "C" fn table_row_get_column_with_ordinal(
    this: *const TableRow,
    ordinal: size_t,
    out: *mut *mut TableColumnRef,
) -> i32 {
    if this.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }

    let result = (&*this).try_column(ordinal);
    let col_ref: TableColumnRef = match result {
        Err(err) => return datajoint_core_set_last_error(err) as i32,
        Ok(value) => value,
    };

    util::mem::handle_output_ptr(out, col_ref);
    ErrorCode::Success as i32
}
