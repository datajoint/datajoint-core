use datajoint_core::blob::{Blob};
use datajoint_core::types::NativeType;


#[no_mangle]
pub unsafe extern "C" fn pack<T>(obj: *mut T){
    

    //let blob = Blob{
    //    packed: Blob::pack(obj);
    //}

    //return Box:into_raw(Blob)
    //let connection = unsafe { &mut *this };
    //blob.unpack(this, packed_blob);
    //blob:Pack();
    
}

#[no_mangle]
pub unsafe extern "C" fn unpack(this: *mut Blob) -> *mut UnBlob{
    let vec = Box::from_raw(this).packed;
    Blob::unpack(vec);




    Box::from_raw(unpack(Box::from_raw(this).packed));
    let unblob = UnBlob{
        result: Blob::unpack(this.packed);
    }
    return unblob
}