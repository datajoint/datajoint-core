use core::mem;
use datajoint_core::placeholders::{PlaceholderArgument, PlaceholderArgumentVector};
use datajoint_core::types::{DataJointType, DecodeResult};
use std::ffi::CStr;
use std::fmt::Error;
use std::os::raw::c_void;

use crate::types::decode::NativeDecodedType;

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

/// PlaceholderArgument* placeholder_argument_vector_add(PlaceholderArgumentVector* self, void* data, size_t data_size, DataJointType type);
// Adds a new placeholder argument to the vector.
/// Data is referenced by the void* `data` and is `data_size` bytes.
/// The data is NOT owned and must remain alive until the placeholder arguments are bound to the query.
/// Data is decoded in the library of type `type`, which is a supported column type for decoding.
/// Returns the created argument for further modification if desired.

//TODO add null checking to this and move placeholder argument to be a variable
#[no_mangle]
pub unsafe extern "C" fn placeholder_argument_vector_add(
    this: *mut PlaceholderArgumentVector,
    data: *mut c_void,
    data_size: usize,
    data_type: NativeDecodedType,
) -> *mut PlaceholderArgument {
    let vector = &mut *this;
    let encoded = data_type.encode(data, data_size);
    let arg = PlaceholderArgument::new(encoded);
    vector.add_arg(arg);
    let last = vector.vec.len() - 1;
    return &mut vector.vec[last];
}
