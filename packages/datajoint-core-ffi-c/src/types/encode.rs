use std::{ffi::CStr, mem};

use libc::c_void;

use crate::datajoint_core::types::DecodeResult;

use super::decode::NativeDecodedType;

impl NativeDecodedType {
    pub unsafe fn encode(&self, data: *mut c_void, data_size: usize) -> DecodeResult {
        match self {
            NativeDecodedType::None => {
                // TODO(Edward Garmon): Need to add support for python None
                DecodeResult::String("None - not supported".to_string())
            }
            NativeDecodedType::Int8 => DecodeResult::Int8(*data.cast::<i8>()),
            NativeDecodedType::UInt8 => DecodeResult::UInt8(*data.cast::<u8>()),
            NativeDecodedType::Int16 => DecodeResult::Int16(*data.cast::<i16>()),
            NativeDecodedType::UInt16 => DecodeResult::UInt16(*data.cast::<u16>()),
            NativeDecodedType::Int32 => DecodeResult::Int32(*data.cast::<i32>()),
            NativeDecodedType::UInt32 => DecodeResult::UInt32(*data.cast::<u32>()),
            NativeDecodedType::String => {
                let str = CStr::from_ptr(data as *const _).to_string_lossy();
                DecodeResult::String(str.to_string())
            }
            NativeDecodedType::Float32 => DecodeResult::Float32(*data.cast::<f32>()),
            NativeDecodedType::Float64 => DecodeResult::Float64(*data.cast::<f64>()),
            NativeDecodedType::Bytes => {
                let mut v = vec![];
                let mut a = data;
                let mut i = 0;

                while i < data_size {
                    v.push(*(a as *mut u8));
                    a = a.add(mem::size_of::<u8>());
                    i += mem::size_of::<u8>();
                }

                DecodeResult::Bytes(v)
            }
        }
    }
}
