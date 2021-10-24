use datajoint_core::error::{Error, ErrorCode};
use libc::c_char;
use std::cell::RefCell;
use std::ffi::CString;

thread_local! {
    /// The last error in the current thread.
    pub static LAST_ERROR: RefCell<Option<Error>> = RefCell::new(None);
}

/// Sets the last library error for the current thread.
///
/// Returns the error code of the new error for easy returns.
pub fn datajoint_core_set_last_error(error: Error) -> ErrorCode {
    let out = error.code();
    LAST_ERROR.with(|last_error| last_error.replace(Some(error)));
    return out;
}

/// Returns the last error message as a C string. Returns null if there has been no error.
///
/// Returned string must be properly deallocated using `datajoint_core_cstring_free`.
#[no_mangle]
pub extern "C" fn datajoint_core_get_last_error_message() -> *const c_char {
    LAST_ERROR.with(|last_error| match &*(last_error.borrow()) {
        None => std::ptr::null(),
        Some(error) => match CString::new(&*error.message()) {
            Err(_) => std::ptr::null(),
            Ok(string) => string.into_raw(),
        },
    })
}

/// Returns the last error code. Returns `ErrorCode::Success` if there has been no error.
#[no_mangle]
pub extern "C" fn datajoint_core_get_last_error_code() -> i32 {
    LAST_ERROR.with(|last_error| match &*(last_error.borrow()) {
        None => ErrorCode::Success as i32,
        Some(error) => error.code() as i32,
    })
}
