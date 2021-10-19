use datajoint_core::placeholders::{PlaceholderArgument, PlaceholderArgumentVector};
use std::os::raw::c_void;
use datajoint_core::types::{DataJointType, DecodeResult};
use std::ffi::CStr;
use core::mem;

// Creates a new placeholder argument vector to send to a query method.
#[no_mangle]
pub extern "C" fn placeholder_argument_vector_new() -> *mut PlaceholderArgumentVector {
    Box::into_raw(Box::new(PlaceholderArgumentVector::new(vec!())))
    
}

// Frees an entire placeholder argument vector, including all argument inside.
#[no_mangle]
pub extern "C" fn placeholder_argument_vector_free(ptr: *mut PlaceholderArgumentVector) {
    if ptr.is_null(){
        return;
    }

    let ph_vec: Box<PlaceholderArgumentVector> = unsafe {
        Box::from_raw(ptr)
    };

    // TODO will this actually free this data
    for arg in ph_vec.vec {
        arg;
    }
}

// PlaceholderArgument* placeholder_argument_vector_add(PlaceholderArgumentVector* self, void* data, size_t data_size, DataJointType type);
// Adds a new placeholder argument to the vector.
// Data is referenced by the void* `data` and is `data_size` bytes.
// The data is NOT owned and must remain alive until the placeholder arguments are bound to the query.
// Data is decoded in the library of type `type`, which is a supported column type for decoding.
// Returns the created argument for further modification if desired.
#[no_mangle]
pub unsafe extern "C" fn placeholder_argument_vector_add(
    this: *mut PlaceholderArgumentVector,
    data: *mut c_void,
    data_size: usize,
    data_type: DataJointType,
) -> *mut PlaceholderArgument{
    let this = &mut*this;

    match data_type {

        //TODO Add error code to deal with unsupported database types passed in.
        DataJointType::Unknown => {  
            return Box::into_raw(Box::new(PlaceholderArgument::new(DecodeResult::String("type not supported".to_string()))))
        }
        DataJointType::Decimal |DataJointType::FilepathStore | DataJointType::Attach => {
            return Box::into_raw(Box::new(PlaceholderArgument::new(DecodeResult::String("type not supported".to_string()))))
        }
        DataJointType::TinyInt => {
            let arg = PlaceholderArgument::new(DecodeResult::Int8( data as i8));
            this.add_arg(arg);
            let last = this.vec.len() -1;
            return &mut this.vec[last];
        },
        DataJointType::TinyIntUnsigned => {
            let arg = PlaceholderArgument::new(DecodeResult::UInt8( data as u8));
            this.add_arg(arg);
            let last = this.vec.len() -1;
            return &mut this.vec[last];
        },
        DataJointType::SmallInt => {
            let arg = Box::new(
                PlaceholderArgument::new(DecodeResult::Int16( data as i16)));
            this.add_arg(*arg);
            let last = this.vec.len() -1;
            return &mut this.vec[last];
        },
        DataJointType::SmallIntUnsigned => {
            let arg = Box::new(
                PlaceholderArgument::new(DecodeResult::Int16( data as i16)));
            this.add_arg(*arg);
            let last = this.vec.len() -1;
            return &mut this.vec[last];;
        },
        DataJointType::MediumInt | DataJointType::Int => {
            let data = data as *mut i32;
            let arg = Box::new(
                PlaceholderArgument::new(DecodeResult::Int32(*data)));
            this.add_arg(*arg);
            let last = this.vec.len() -1;
            return &mut this.vec[last];
        },
        DataJointType::MediumIntUnsigned | DataJointType::IntUnsigned
        => {
            let arg = Box::new(PlaceholderArgument::new(DecodeResult::UInt16( data as u16)));
            this.add_arg(*arg);
            let last = this.vec.len() -1;
            return &mut this.vec[last];
        }
        DataJointType::Enum | DataJointType::CharN | DataJointType::VarCharN
        => {
            let str = CStr::from_ptr(data as *const _).to_string_lossy();
            let arg = Box::new(PlaceholderArgument::new(DecodeResult::String(str.to_string())));
            this.add_arg(*arg);
            let last = this.vec.len() -1;
            return &mut this.vec[last];
        }
        DataJointType::Date => {
            let str = CStr::from_ptr(data as *const _).to_string_lossy();
            let arg = Box::new(PlaceholderArgument::new(DecodeResult::String(str.to_string())));
            this.add_arg(*arg);
            let last = this.vec.len() -1;
            return &mut this.vec[last];
        }
        DataJointType::Time => {
            let str = CStr::from_ptr(data as *const _).to_string_lossy();
            let arg = Box::new(PlaceholderArgument::new(DecodeResult::String(str.to_string())));
            this.add_arg(*arg);
            let last = this.vec.len() -1;
            return &mut this.vec[last];
        }
        DataJointType::DateTime => {
            let str = CStr::from_ptr(data as *const _).to_string_lossy();
            let arg = Box::new(PlaceholderArgument::new(DecodeResult::String(str.to_string())));
            this.add_arg(*arg);
            let last = this.vec.len() -1;
            return &mut this.vec[last];
        }
        DataJointType::Timestamp => {
            let str = CStr::from_ptr(data as *const _).to_string_lossy();
            let arg = Box::new(PlaceholderArgument::new(DecodeResult::String(str.to_string())));
            this.add_arg(*arg);
            let last = this.vec.len() -1;
            return &mut this.vec[last];
        }
        DataJointType::Float => {
            let arg = Box::new(
                PlaceholderArgument::new(DecodeResult::Float32( *data.cast::<f32>())));
            this.add_arg(*arg);
            let last = this.vec.len() -1;
            return &mut this.vec[last];
        }
        DataJointType::Double => {
            let arg = Box::new(
                PlaceholderArgument::new(DecodeResult::Float64( *data.cast::<f64>())));
            this.add_arg(*arg);
            let last = this.vec.len() -1;
            return &mut this.vec[last];
        }
        DataJointType::TinyBlob | DataJointType::MediumBlob |
        DataJointType::Blob | DataJointType::LongBlob
        => {
            let mut v = vec![];
            let a = data;
            let mut i = 0;
            
            while i < data_size{
                v.push(*(a as * mut u8));
                a.add(mem::size_of::<u8>());
                i += mem::size_of::<u8>();
            }

            let data = DecodeResult::Bytes(v);
            let arg = Box::new(PlaceholderArgument::new(data));
            this.add_arg(*arg);
            let last = this.vec.len() -1;
            return &mut this.vec[last];
        }

    }
}


#[no_mangle]
pub unsafe extern "C" fn placeholder_argument_vector_print_args(
    this: *mut PlaceholderArgumentVector,
){
    let this = &* this;
    for arg in &this.vec {
        println!("{:?}", &arg.arg);
    }
}


