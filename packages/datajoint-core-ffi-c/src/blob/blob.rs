use datajoint_core::blob::{Blob};

#[no_mangle]
pub unsafe extern "C" fn packInt(this: i64) -> *mut Vec<u8>{

    let blob = Blob::pack(this);
    return Box::into_raw(Box::new(blob));
}

#[no_mangle]
pub unsafe extern "C" fn packFloat(this: f64) -> *mut Vec<u8>{

    let blob = Blob::pack(this);
    return Box::into_raw(Box::new(blob));
}

#[no_mangle]
pub unsafe extern "C" fn packString(this: String) -> *mut Vec<u8>{

    let blob = Blob::pack(this);
    return Box::into_raw(Box::new(blob));
}

#[no_mangle]
pub unsafe extern "C" fn packBool(this: bool) -> *mut Vec<u8>{

    let blob = Blob::pack(this);
    return Box::into_raw(Box::new(blob));
}

/*
#[no_mangle]
pub unsafe extern "C" fn packdict(this: ) -> *mut Vec<u8>{
    let blob = Blob::pack(this);
    return Box::into_raw(Box::new(blob));

}
*/

#[no_mangle]
pub unsafe extern "C" fn unpack(this: *mut Vec<u8>){
    let answer = Blob::unpack(*Box::from_raw(this));
    let answer2 = &*this;
    println!("{:?} \n{:?}", answer, answer2);
}
