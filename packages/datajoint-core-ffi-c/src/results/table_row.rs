use datajoint_core::error::ErrorCode;
use datajoint_core::results::TableColumnRef;
use datajoint_core::results::TableRow;
use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
pub unsafe extern "C" fn table_row_new() -> *mut TableRow {
    libc::malloc(std::mem::size_of::<TableRow> as libc::size_t) as *mut TableRow
}

#[no_mangle]
pub extern "C" fn table_row_free(this: *mut TableRow) {
    if this.is_null() {
        return;
    }
    unsafe {
        libc::free(this as *mut libc::c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn table_row_is_empty(this: *const TableRow) -> i32 {
    if this.is_null() {
        return ErrorCode::NullNotAllowed as i32;
    }
    (&*this).is_empty() as i32
}

#[no_mangle]
pub unsafe extern "C" fn table_row_column_count(this: *const TableRow) -> usize {
    if this.is_null() {
        return 0;
    }
    (&*this).column_count()
}

#[no_mangle]
pub unsafe extern "C" fn table_row_columns(
    this: *const TableRow,
    out_columns: *mut *const TableColumnRef,
    columns_size: *mut usize,
) -> i32 {
    if this.is_null() {
        return ErrorCode::NullNotAllowed as i32;
    }
    let mut cols = (&*this).columns();
    cols.shrink_to_fit();
    *columns_size = cols.len();
    *out_columns = cols.as_mut_ptr();
    std::mem::forget(cols);
    ErrorCode::Success as i32
}

#[no_mangle]
pub unsafe extern "C" fn table_row_columns_free(
    out_columns: *mut TableColumnRef,
    columns_size: usize,
) {
    if out_columns.is_null() {
        return;
    }
    drop(Vec::from_raw_parts(out_columns, columns_size, columns_size));
}

#[no_mangle]
pub unsafe extern "C" fn table_row_get_column_with_name(
    this: *const TableRow,
    column_name: *const c_char,
    mut _out: *mut TableColumnRef,
) -> i32 {
    if this.is_null() || column_name.is_null() {
        return ErrorCode::NullNotAllowed as i32;
    }

    let column_name = CStr::from_ptr(column_name);
    let result = column_name.to_str();

    let column_name_str = result.unwrap_or("");

    let result = (&*this).try_column(column_name_str);
    let col_ref: TableColumnRef = match result {
        Err(_err) => return ErrorCode::ColumnDecodeError as i32,
        Ok(value) => value,
    };
    *(_out) = col_ref;
    ErrorCode::Success as i32
}

#[no_mangle]
pub unsafe extern "C" fn table_row_get_column_with_ordinal(
    this: *const TableRow,
    ordinal: usize,
    mut _out: *mut TableColumnRef,
) -> i32 {
    if this.is_null() {
        return ErrorCode::NullNotAllowed as i32;
    }

    let result = (&*this).try_column(ordinal);
    let col_ref: TableColumnRef = match result {
        Err(_err) => return ErrorCode::ColumnDecodeError as i32,
        Ok(value) => value,
    };

    *(_out) = col_ref;

    ErrorCode::Success as i32
}
