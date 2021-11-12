use datajoint_core::results::TableColumnRef;
use datajoint_core::types::DataJointType;
use libc::{c_char, size_t};
use std::ffi::CString;

/// Frees a table column reference.
#[no_mangle]
pub unsafe extern "C" fn table_column_ref_free<'r>(this: *mut TableColumnRef<'r>) {
    if !this.is_null() {
        Box::from_raw(this);
    }
}

/// Gives the integer ordinal of the column, which can be used to
/// fetch the column in a row.
#[no_mangle]
pub extern "C" fn table_column_ref_ordinal<'r>(this: *const TableColumnRef<'r>) -> size_t {
    if this.is_null() {
        return 0;
    }
    let column = unsafe { &*this };
    column.ordinal()
}

/// Gives the name of the column, which can be used to fetch the
/// column in a row.
#[no_mangle]
pub extern "C" fn table_column_ref_name<'r>(this: *const TableColumnRef<'r>) -> *const c_char {
    if this.is_null() {
        return std::ptr::null();
    }
    let column = unsafe { &*this };
    match CString::new(column.name()) {
        Err(_) => std::ptr::null(),
        Ok(string) => string.into_raw(),
    }
}

/// The DataJoint type for the column.
#[no_mangle]
pub extern "C" fn table_column_ref_type<'r>(this: *const TableColumnRef<'r>) -> DataJointType {
    if this.is_null() {
        return DataJointType::Unknown;
    }
    let column = unsafe { &*this };
    column.type_name()
}
