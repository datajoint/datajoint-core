use crate::error::datajoint_core_set_last_error;
use crate::types::native_type::NativeTypeEnum;
use datajoint_core::error::{DataJointError, ErrorCode};
use datajoint_core::results::{TableColumnRef, TableRow};
use datajoint_core::types::NativeType;
use libc::size_t;
use std::ffi::{c_void, CString};
use std::os::raw::c_char;

/// Decodes a single table row value to a caller-allocated buffer.
///
/// The caller is responsible for moving data out of the buffer and handling
/// the deallocation of the buffer itself.
#[no_mangle]
pub unsafe extern "C" fn table_row_decode_to_buffer(
    this: *const TableRow,
    column: *const TableColumnRef,
    buffer: *mut c_void,
    buffer_size: size_t,
    output_size: *mut size_t,
    output_type: *mut NativeTypeEnum,
) -> i32 {
    if this.is_null() || column.is_null() || buffer.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }
    match (*this).try_decode_optional(*column) {
        Err(err) => datajoint_core_set_last_error(err) as i32,
        Ok(None) => {
            *output_size = 0;
            *output_type = NativeTypeEnum::Null;
            ErrorCode::Success as i32
        }
        Ok(Some(result)) => match result {
            NativeType::None => ErrorCode::ValueDecodeError as i32,
            // No macro for generating these because of cbindgen limitations.
            NativeType::Bool(value) => {
                // Check that buffer is large enough.
                if buffer_size < std::mem::size_of::<bool>() {
                    return datajoint_core_set_last_error(DataJointError::new(
                        ErrorCode::BufferNotEnough,
                    )) as i32;
                }

                // Move the data into the buffer.
                *(buffer as *mut bool) = value;

                // Set output variables if allowed.
                if !output_size.is_null() {
                    *output_size = std::mem::size_of::<bool>();
                }
                if !output_type.is_null() {
                    *output_type = NativeTypeEnum::Int8;
                }
                ErrorCode::Success as i32
            }
            NativeType::Int8(value) => {
                // Check that buffer is large enough.
                if buffer_size < std::mem::size_of::<i8>() {
                    return datajoint_core_set_last_error(DataJointError::new(
                        ErrorCode::BufferNotEnough,
                    )) as i32;
                }

                // Move the data into the buffer.
                *(buffer as *mut i8) = value;

                // Set output variables if allowed.
                if !output_size.is_null() {
                    *output_size = std::mem::size_of::<i8>();
                }
                if !output_type.is_null() {
                    *output_type = NativeTypeEnum::Int8;
                }
                ErrorCode::Success as i32
            }
            NativeType::UInt8(value) => {
                // Check that buffer is large enough.
                if buffer_size < std::mem::size_of::<u8>() {
                    return datajoint_core_set_last_error(DataJointError::new(
                        ErrorCode::BufferNotEnough,
                    )) as i32;
                }

                // Move the data into the buffer.
                *(buffer as *mut u8) = value;

                // Set output variables if allowed.
                if !output_size.is_null() {
                    *output_size = std::mem::size_of::<u8>();
                }
                if !output_type.is_null() {
                    *output_type = NativeTypeEnum::UInt8;
                }
                ErrorCode::Success as i32
            }
            NativeType::Int16(value) => {
                // Check that buffer is large enough.
                if buffer_size < std::mem::size_of::<i16>() {
                    return datajoint_core_set_last_error(DataJointError::new(
                        ErrorCode::BufferNotEnough,
                    )) as i32;
                }

                // Move the data into the buffer.
                *(buffer as *mut i16) = value;

                // Set output variables if allowed.
                if !output_size.is_null() {
                    *output_size = std::mem::size_of::<i16>();
                }
                if !output_type.is_null() {
                    *output_type = NativeTypeEnum::Int16;
                }
                ErrorCode::Success as i32
            }
            NativeType::UInt16(value) => {
                // Check that buffer is large enough.
                if buffer_size < std::mem::size_of::<u16>() {
                    return datajoint_core_set_last_error(DataJointError::new(
                        ErrorCode::BufferNotEnough,
                    )) as i32;
                }

                // Move the data into the buffer.
                *(buffer as *mut u16) = value;

                // Set output variables if allowed.
                if !output_size.is_null() {
                    *output_size = std::mem::size_of::<u16>();
                }
                if !output_type.is_null() {
                    *output_type = NativeTypeEnum::UInt16;
                }
                ErrorCode::Success as i32
            }
            NativeType::Int32(value) => {
                // Check that buffer is large enough.
                if buffer_size < std::mem::size_of::<i32>() {
                    return datajoint_core_set_last_error(DataJointError::new(
                        ErrorCode::BufferNotEnough,
                    )) as i32;
                }

                // Move the data into the buffer.
                *(buffer as *mut i32) = value;

                // Set output variables if allowed.
                if !output_size.is_null() {
                    *output_size = std::mem::size_of::<i32>();
                }
                if !output_type.is_null() {
                    *output_type = NativeTypeEnum::Int32;
                }
                ErrorCode::Success as i32
            }
            NativeType::UInt32(value) => {
                // Check that buffer is large enough.
                if buffer_size < std::mem::size_of::<u32>() {
                    return datajoint_core_set_last_error(DataJointError::new(
                        ErrorCode::BufferNotEnough,
                    )) as i32;
                }

                // Move the data into the buffer.
                *(buffer as *mut u32) = value;

                // Set output variables if allowed.
                if !output_size.is_null() {
                    *output_size = std::mem::size_of::<u32>();
                }
                if !output_type.is_null() {
                    *output_type = NativeTypeEnum::UInt32;
                }
                ErrorCode::Success as i32
            }
            NativeType::Int64(value) => {
                // Check that buffer is large enough.
                if buffer_size < std::mem::size_of::<i64>() {
                    return datajoint_core_set_last_error(DataJointError::new(
                        ErrorCode::BufferNotEnough,
                    )) as i32;
                }

                // Move the data into the buffer.
                *(buffer as *mut i64) = value;

                // Set output variables if allowed.
                if !output_size.is_null() {
                    *output_size = std::mem::size_of::<i64>();
                }
                if !output_type.is_null() {
                    *output_type = NativeTypeEnum::Int64;
                }
                ErrorCode::Success as i32
            }
            NativeType::UInt64(value) => {
                // Check that buffer is large enough.
                if buffer_size < std::mem::size_of::<u64>() {
                    return datajoint_core_set_last_error(DataJointError::new(
                        ErrorCode::BufferNotEnough,
                    )) as i32;
                }

                // Move the data into the buffer.
                *(buffer as *mut u64) = value;

                // Set output variables if allowed.
                if !output_size.is_null() {
                    *output_size = std::mem::size_of::<u64>();
                }
                if !output_type.is_null() {
                    *output_type = NativeTypeEnum::UInt64;
                }
                ErrorCode::Success as i32
            }
            NativeType::Float32(value) => {
                // Check that buffer is large enough.
                if buffer_size < std::mem::size_of::<f32>() {
                    return datajoint_core_set_last_error(DataJointError::new(
                        ErrorCode::BufferNotEnough,
                    )) as i32;
                }

                // Move the data into the buffer.
                *(buffer as *mut f32) = value;

                // Set output variables if allowed.
                if !output_size.is_null() {
                    *output_size = std::mem::size_of::<f32>();
                }
                if !output_type.is_null() {
                    *output_type = NativeTypeEnum::Float32;
                }
                ErrorCode::Success as i32
            }
            NativeType::Float64(value) => {
                // Check that buffer is large enough.
                if buffer_size < std::mem::size_of::<f64>() {
                    return datajoint_core_set_last_error(DataJointError::new(
                        ErrorCode::BufferNotEnough,
                    )) as i32;
                }

                // Move the data into the buffer.
                *(buffer as *mut f64) = value;

                // Set output variables if allowed.
                if !output_size.is_null() {
                    *output_size = std::mem::size_of::<f64>();
                }
                if !output_type.is_null() {
                    *output_type = NativeTypeEnum::Float64;
                }
                ErrorCode::Success as i32
            }
            NativeType::String(string) => {
                if buffer_size == 0 {
                    return datajoint_core_set_last_error(DataJointError::new(
                        ErrorCode::BufferNotEnough,
                    )) as i32;
                }

                // Can write at most buffer_size - 1 chars to assure the trailing null
                // char is also placed in the buffer.
                let write_size = std::cmp::min(string.len(), buffer_size - 1);

                // Copy string bytes to buffer bytes.
                let buffer_bytes = std::slice::from_raw_parts_mut(buffer as *mut u8, buffer_size);
                buffer_bytes[..write_size].copy_from_slice(string[..write_size].as_bytes());

                // Trailing null character.
                buffer_bytes[write_size] = 0;

                if !output_size.is_null() {
                    // Trailing null is NOT accounted for in output size.
                    *output_size = write_size;
                }
                if !output_type.is_null() {
                    *output_type = NativeTypeEnum::String;
                }
                ErrorCode::Success as i32
            }
            NativeType::Bytes(bytes) => {
                if buffer_size == 0 {
                    return datajoint_core_set_last_error(DataJointError::new(
                        ErrorCode::BufferNotEnough,
                    )) as i32;
                }

                let write_size = std::cmp::min(bytes.len(), buffer_size);
                let buffer_bytes = std::slice::from_raw_parts_mut(buffer as *mut u8, buffer_size);
                buffer_bytes[..write_size].copy_from_slice(&bytes[..write_size]);

                if !output_size.is_null() {
                    *output_size = write_size;
                }
                if !output_type.is_null() {
                    *output_type = NativeTypeEnum::Bytes;
                }
                ErrorCode::Success as i32
            }
        },
    }
}

/// A single decoded value that has been allocated by the core library.
///
/// This struct wraps a value allocated to be transmitted to C. It allows
/// the value to be decoded to a native type by the caller.
pub struct AllocatedDecodedValue {
    pub(crate) data: *const c_void,
    pub(crate) size: usize,
    pub(crate) type_name: NativeTypeEnum,
}

impl AllocatedDecodedValue {
    /// Creates a new allocated decoded value.
    ///
    /// Does not allocate any internal value.
    pub fn new() -> Self {
        AllocatedDecodedValue {
            data: std::ptr::null(),
            size: 0,
            type_name: NativeTypeEnum::None,
        }
    }

    /// Resets the wrapper by deallocating the memory.
    pub unsafe fn reset(&mut self) {
        // The integrity of self.type_name should be preserved to assure
        // these type casts work!
        //
        // This value cannot be set, by any means, by the outside world.
        match self.type_name {
            NativeTypeEnum::None | NativeTypeEnum::Null => (),
            NativeTypeEnum::Bool => {
                Box::from_raw(self.data as *mut bool);
            }
            NativeTypeEnum::Int8 => {
                Box::from_raw(self.data as *mut i8);
            }
            NativeTypeEnum::UInt8 => {
                Box::from_raw(self.data as *mut u8);
            }
            NativeTypeEnum::Int16 => {
                Box::from_raw(self.data as *mut i16);
            }
            NativeTypeEnum::UInt16 => {
                Box::from_raw(self.data as *mut u16);
            }
            NativeTypeEnum::Int32 => {
                Box::from_raw(self.data as *mut i32);
            }
            NativeTypeEnum::UInt32 => {
                Box::from_raw(self.data as *mut u32);
            }
            NativeTypeEnum::Int64 => {
                Box::from_raw(self.data as *mut i64);
            }
            NativeTypeEnum::UInt64 => {
                Box::from_raw(self.data as *mut u64);
            }
            NativeTypeEnum::Float32 => {
                Box::from_raw(self.data as *mut f32);
            }
            NativeTypeEnum::Float64 => {
                Box::from_raw(self.data as *mut f64);
            }
            NativeTypeEnum::String => {
                CString::from_raw(self.data as *mut c_char);
            }
            NativeTypeEnum::Bytes => {
                Box::from_raw(self.data as *mut u8);
            }
        }
        self.size = 0;
        self.type_name = NativeTypeEnum::None;
    }
}

/// Creates instance of AllocatedDecodedValue.
#[no_mangle]
pub extern "C" fn allocated_decoded_value_new() -> *mut AllocatedDecodedValue {
    Box::into_raw(Box::new(AllocatedDecodedValue::new()))
}

/// Frees instance of AllocatedDecodedValue
#[no_mangle]
pub unsafe extern "C" fn allocated_decoded_value_free(this: *mut AllocatedDecodedValue) {
    if !this.is_null() {
        (*this).reset();
        Box::from_raw(this);
    }
}

/// Returns the data of the AllocatedDecodedValue.
#[no_mangle]
pub unsafe extern "C" fn allocated_decoded_value_data(
    this: *const AllocatedDecodedValue,
) -> *const c_void {
    if this.is_null() {
        std::ptr::null()
    } else {
        (*this).data
    }
}

/// Returns the size of the AllocatedDecodedValue.
#[no_mangle]
pub unsafe extern "C" fn allocated_decoded_value_size(
    this: *const AllocatedDecodedValue,
) -> size_t {
    if this.is_null() {
        return 0;
    } else {
        (*this).size
    }
}

/// Returns the type_name of the AllocatedDecodedValue.
#[no_mangle]
pub unsafe extern "C" fn allocated_decoded_value_type(
    this: *const AllocatedDecodedValue,
) -> NativeTypeEnum {
    if this.is_null() {
        return NativeTypeEnum::None;
    } else {
        (*this).type_name
    }
}

#[no_mangle]
pub unsafe extern "C" fn allocated_decoded_value_is_null_value(
    this: *const AllocatedDecodedValue,
) -> i32 {
    (this.is_null() || (*this).type_name == NativeTypeEnum::Null) as i32
}

/// Decodes a single table row value to a Rust-allocated buffer stored in a
/// caller-allocated wrapper value.
///
/// The caller is responsible for moving data out of the buffer and handling
/// the deallocation of the wrapper. When the wrapper is deallocated, the
/// data inside is properly deallocated depending on the type.
#[no_mangle]
pub extern "C" fn table_row_decode_to_allocation(
    this: *const TableRow,
    column: *const TableColumnRef,
    value: *mut AllocatedDecodedValue,
) -> i32 {
    if this.is_null() || column.is_null() || value.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }

    unsafe {
        (*value).reset();
        match (*this).try_decode_optional(*column) {
            Err(err) => datajoint_core_set_last_error(err) as i32,
            Ok(None) => {
                (*value).type_name = NativeTypeEnum::Null;
                ErrorCode::Success as i32
            }
            Ok(Some(res)) => match res {
                NativeType::None => {
                    datajoint_core_set_last_error(DataJointError::new(ErrorCode::ValueDecodeError))
                        as i32
                }
                // No macro for generating these because of cbindgen limitations.
                NativeType::Bool(data) => {
                    (*value).data = Box::into_raw(Box::new(data)) as *mut c_void;
                    (*value).size = std::mem::size_of::<bool>();
                    (*value).type_name = NativeTypeEnum::Bool;
                    ErrorCode::Success as i32
                }
                NativeType::Int8(data) => {
                    (*value).data = Box::into_raw(Box::new(data)) as *mut c_void;
                    (*value).size = std::mem::size_of::<i8>();
                    (*value).type_name = NativeTypeEnum::Int8;
                    ErrorCode::Success as i32
                }
                NativeType::UInt8(data) => {
                    (*value).data = Box::into_raw(Box::new(data)) as *mut c_void;
                    (*value).size = std::mem::size_of::<u8>();
                    (*value).type_name = NativeTypeEnum::UInt8;
                    ErrorCode::Success as i32
                }
                NativeType::Int16(data) => {
                    (*value).data = Box::into_raw(Box::new(data)) as *mut c_void;
                    (*value).size = std::mem::size_of::<i16>();
                    (*value).type_name = NativeTypeEnum::Int16;
                    ErrorCode::Success as i32
                }
                NativeType::UInt16(data) => {
                    (*value).data = Box::into_raw(Box::new(data)) as *mut c_void;
                    (*value).size = std::mem::size_of::<u16>();
                    (*value).type_name = NativeTypeEnum::UInt16;
                    ErrorCode::Success as i32
                }
                NativeType::Int32(data) => {
                    (*value).data = Box::into_raw(Box::new(data)) as *mut c_void;
                    (*value).size = std::mem::size_of::<i32>();
                    (*value).type_name = NativeTypeEnum::Int32;
                    ErrorCode::Success as i32
                }
                NativeType::UInt32(data) => {
                    (*value).data = Box::into_raw(Box::new(data)) as *mut c_void;
                    (*value).size = std::mem::size_of::<u32>();
                    (*value).type_name = NativeTypeEnum::UInt32;
                    ErrorCode::Success as i32
                }
                NativeType::Int64(data) => {
                    (*value).data = Box::into_raw(Box::new(data)) as *mut c_void;
                    (*value).size = std::mem::size_of::<i64>();
                    (*value).type_name = NativeTypeEnum::Int64;
                    ErrorCode::Success as i32
                }
                NativeType::UInt64(data) => {
                    (*value).data = Box::into_raw(Box::new(data)) as *mut c_void;
                    (*value).size = std::mem::size_of::<u64>();
                    (*value).type_name = NativeTypeEnum::UInt64;
                    ErrorCode::Success as i32
                }
                NativeType::Float32(data) => {
                    (*value).data = Box::into_raw(Box::new(data)) as *mut c_void;
                    (*value).size = std::mem::size_of::<f32>();
                    (*value).type_name = NativeTypeEnum::Float32;
                    ErrorCode::Success as i32
                }
                NativeType::Float64(data) => {
                    (*value).data = Box::into_raw(Box::new(data)) as *mut c_void;
                    (*value).size = std::mem::size_of::<f64>();
                    (*value).type_name = NativeTypeEnum::Float64;
                    ErrorCode::Success as i32
                }
                NativeType::String(string) => {
                    (*value).size = string.len();
                    (*value).type_name = NativeTypeEnum::String;
                    match CString::new(string) {
                        Err(_) => datajoint_core_set_last_error(DataJointError::new(
                            ErrorCode::InvalidUtf8String,
                        )) as i32,
                        Ok(cstr) => {
                            (*value).data = cstr.into_raw() as *const c_void;
                            ErrorCode::Success as i32
                        }
                    }
                }
                NativeType::Bytes(bytes) => {
                    (*value).size = bytes.len();
                    (*value).type_name = NativeTypeEnum::Bytes;
                    (*value).data = Box::into_raw(Box::new(bytes)) as *const c_void;
                    ErrorCode::Success as i32
                }
            },
        }
    }
}
