use datajoint_core::blob::{Blob};
use std::ffi::CStr;
use serde_json::json;
use std::os::raw::c_char;
use std::str;

// #[repr(C)]
// pub enum PyObject{
//     Type1(i64),
//     Type2(f64),
//     Type3(bool),
//     Type4(*const c_char),
// }


// #[no_mangle]
// pub unsafe extern "C" fn pack(this: PyObject){
    
// }


#[no_mangle]
pub unsafe extern "C" fn packInt(this: i64) -> *mut Blob{
    let blob = Blob{
        blobbed: Blob::pack(json!(this)),
    };
    return Box::into_raw(Box::new(blob));
}

#[no_mangle]
pub unsafe extern "C" fn packFloat(this: f64) -> *mut Blob{
    let blob = Blob{
        blobbed: Blob::pack(json!(this)),
    };
    return Box::into_raw(Box::new(blob));
}

#[no_mangle]
pub unsafe extern "C" fn packBool(this: bool) -> *mut Blob{
    println!("{}", this);
    let blob = Blob{
        blobbed: Blob::pack(json!(this)),
    };
    return Box::into_raw(Box::new(blob));
}


#[no_mangle]
pub unsafe extern "C" fn packdict(this: *mut serde_json::value::Value) -> *mut Blob{
    let this = *Box::from_raw(this);
    let blob = Blob{
        blobbed: Blob::pack(json!(this)),
    };
    return Box::into_raw(Box::new(blob));
}


#[no_mangle]
pub unsafe extern "C" fn packString(this: *const c_char ) -> *mut Blob{
    let c_str = CStr::from_ptr(this);
    let r_str = c_str.to_str().unwrap();
    let blob = Blob{
        blobbed: Blob::pack(json!(r_str)),
    };
    return Box::into_raw(Box::new(blob));
}

#[no_mangle]
pub unsafe extern "C" fn unpack(this: *mut Blob){
    let answer = Blob::unpack((*Box::from_raw(this).blobbed).to_vec());
    let answer2 = &*this;
    println!("{:?} \n{:02x?}", answer, answer2.blobbed);
}

