use crate::datajoint_core::types::NativeType;
use crate::types::native_type::NativeTypeEnum;
use datajoint_core::error::{DataJointError, Error, ErrorCode};
use libc::c_void;
use std::ffi::CStr;

impl NativeTypeEnum {
    /// Encodes raw native type data into the proper enum variant.
    pub unsafe fn encode(&self, data: *mut c_void, data_size: usize) -> Result<NativeType, Error> {
        match self {
            NativeTypeEnum::None => Ok(NativeType::None),
            NativeTypeEnum::Int8 => Ok(NativeType::Int8(*data.cast::<i8>())),
            NativeTypeEnum::UInt8 => Ok(NativeType::UInt8(*data.cast::<u8>())),
            NativeTypeEnum::Int16 => Ok(NativeType::Int16(*data.cast::<i16>())),
            NativeTypeEnum::UInt16 => Ok(NativeType::UInt16(*data.cast::<u16>())),
            NativeTypeEnum::Int32 => Ok(NativeType::Int32(*data.cast::<i32>())),
            NativeTypeEnum::UInt32 => Ok(NativeType::UInt32(*data.cast::<u32>())),
            NativeTypeEnum::String => {
                let str = match CStr::from_ptr(data as *const _).to_str() {
                    Err(_) => {
                        return Err(DataJointError::new(
                            "invalid utf-8 string",
                            ErrorCode::InvalidCString,
                        ))
                    }
                    Ok(str) => str,
                };
                Ok(NativeType::String(str.to_string()))
            }
            NativeTypeEnum::Float32 => Ok(NativeType::Float32(*data.cast::<f32>())),
            NativeTypeEnum::Float64 => Ok(NativeType::Float64(*data.cast::<f64>())),
            NativeTypeEnum::Bytes => Ok(NativeType::Bytes(
                std::slice::from_raw_parts(data as *const u8, data_size).to_vec(),
            )),
        }
    }
}
