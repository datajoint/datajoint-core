use crate::types::native_type::NativeTypeEnum;
use datajoint_core::{
    error::ErrorCode,
    placeholders::{PlaceholderArgument, PlaceholderArgumentVector},
};
use std::os::raw::c_void;

/// Creates a new placeholder argument vector to send to a query method.
#[no_mangle]
pub extern "C" fn placeholder_argument_vector_new() -> *mut PlaceholderArgumentVector {
    Box::into_raw(Box::new(PlaceholderArgumentVector::new(vec![])))
}

/// Frees an entire placeholder argument vector, including all argument inside.
#[no_mangle]
pub extern "C" fn placeholder_argument_vector_free(ptr: *mut PlaceholderArgumentVector) {
    if !ptr.is_null() {
        unsafe { Box::from_raw(ptr) };
    }
}

/// Adds a new placeholder argument to the vector.
/// Data is referenced by the void* `data` and is `data_size` bytes.
/// The data is NOT owned and must remain alive until the placeholder arguments are bound to the query.
/// Data is decoded in the library of type `type`, which is a supported column type for decoding.
/// Returns the created argument for further modification if desired.
#[no_mangle]
pub unsafe extern "C" fn placeholder_argument_vector_add(
    this: *mut PlaceholderArgumentVector,
    data: *mut c_void,
    data_size: usize,
    data_type: NativeTypeEnum,
    out: *mut *mut PlaceholderArgument,
) -> i32 {
    let vector = &mut *this;
    let encoded = match data_type.encode(data, data_size) {
        Err(error) => return error.code() as i32,
        Ok(val) => val,
    };

    vector.add_arg(PlaceholderArgument::new(encoded));

    if !out.is_null() {
        let last = vector.vec.len() - 1;
        *out = &mut vector.vec[last] as *mut PlaceholderArgument;
    }

    return ErrorCode::Success as i32;
}
