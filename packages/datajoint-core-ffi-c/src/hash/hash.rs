use datajoint_core::hash::{Hash};
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub unsafe extern "C" fn uuid_from_buffer(bytes: *const c_char) -> *mut c_char{
    let c_str = CStr::from_ptr(bytes);
    let r_str = c_str.to_str().unwrap();
    let answer = Hash::uuid_from_buffer(r_str.as_bytes());
    let result = CString::new(answer).unwrap();
    return result.into_raw();
}

#[no_mangle]
pub unsafe extern "C" fn uuid_from_file(filepath: *const c_char) -> *mut c_char{
    let c_str = CStr::from_ptr(filepath);
    let r_str = c_str.to_str().unwrap();
    let answer = Hash::uuid_from_file(r_str.to_string());
    let result = CString::new(answer).unwrap();
    return result.into_raw();
}