use std::ffi::CString;
use std::os::raw::c_char;

/// Frees a CString that was allocated on the Rust-side of the core library.
#[no_mangle]
pub unsafe extern "C" fn datajoint_core_cstring_free(string: *mut c_char) {
    if string.is_null() {
        return;
    }

    CString::from_raw(string);
}
