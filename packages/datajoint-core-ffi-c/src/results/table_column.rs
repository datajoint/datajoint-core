use datajoint_core::results::TableColumnRef;
use datajoint_core::types::DataJointType;
use libc::{c_char, c_void, size_t};
use std::ffi::CString;

#[no_mangle]
pub unsafe extern "C" fn table_column_ref_new<'r>() -> *mut TableColumnRef<'r> {
    libc::malloc(std::mem::size_of::<TableColumnRef>() as size_t) as *mut TableColumnRef<'r>
}

#[no_mangle]
pub unsafe extern "C" fn table_column_ref_free<'r>(this: *mut TableColumnRef<'r>) {
    if !this.is_null() {
        libc::free(this as *mut c_void);
    }
}

#[no_mangle]
pub extern "C" fn table_column_ref_ordinal<'r>(this: *const TableColumnRef<'r>) -> usize {
    if this.is_null() {
        return 0;
    }
    let column = unsafe { &*this };
    column.ordinal()
}

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

#[no_mangle]
pub extern "C" fn table_column_ref_type<'r>(this: *const TableColumnRef<'r>) -> DataJointType {
    if this.is_null() {
        return DataJointType::Unknown;
    }
    let column = unsafe { &*this };
    column.type_name()
}
