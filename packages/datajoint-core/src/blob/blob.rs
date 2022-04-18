use std::convert::TryInto;
use std::collections::HashMap;
use serde_json::json;
// use crate::error::{DataJointError, Error, ErrorCode};
// use crate::types::NativeType;


#[cfg(test)]
mod test {
    use serde_json::json;
    use super::Blob;

    #[test]
    fn test_blob() {
        let item = json!({
            "key1": "value",
            "key2": ["val", "val", "val"],
            "key3": { "keyX": 12 }
        });
        let x = "hello";
        let mut v = json!({ "an": "object" });
        let var = Blob::pack(item);
        println!("{:02x?}", var);
        println!("{}\n", Blob::unpack(var));
        
        let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
        println!("{:x}", digest);
        assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");

    
        // let response = serde_json::to_string(&item).unwrap();
    }
}

#[repr(C)]
pub struct Blob {
    pub blobbed: Vec<u8>,
}

impl Blob{
    pub fn pack(obj: serde_json::value::Value) -> Vec<u8> {
        let mut blob: Vec<u8> = b"dj0\0".to_vec();
        blob.append(&mut pack_blob(obj));
        return blob;
    }

    pub fn unpack (mut blob: Vec<u8>) -> serde_json::value::Value{
        // Get Protocol
        let pos = get_zero_terminated_pos(&blob);
        blob.remove(pos);
        let mut protocol: Vec<u8> = blob;
        blob = protocol.split_off(pos);
        return read_blob(blob);
    }
}

fn read_blob(mut blob: Vec<u8>) -> serde_json::value::Value{
    // Get Prefix
    let prefix = blob.get(0).cloned().unwrap();
    blob = blob.split_off(1);

    match prefix{
        b'\x02'=> return json!(unpack_vec(blob)),
        b'\x04'=> return json!(unpack_dict(blob)),
        b'\x05'=> return json!(unpack_string(blob)),
        b'\x0a'=> return json!(unpack_int(blob)),
        b'\x0b'=> return json!(unpack_bool(blob)),
        b'\x0d'=> return json!(unpack_float(blob)),
        _=>/*println!("Not Implemented")*/ serde_json::value::Value::Null
    }
}

fn pack_blob(obj: serde_json::value::Value) -> Vec<u8> {

    let packed_data: Vec<u8> = {
        if obj.is_i64() { obj.as_i64().unwrap().pack() }
        else if obj.is_f64() { obj.as_f64().unwrap().pack() }
        else if obj.is_string() { obj.as_str().unwrap().pack() }
        else if obj.is_boolean() {obj.as_bool().unwrap().pack()}
        else if obj.is_array() { pack_vec(obj) }
        else if obj.is_object() { pack_dict(obj) }
        else {vec![b'a']}
    };

    return packed_data;
}

// ----- COMPLEX DATA TYPE IMPLEMENTATION ----- //
fn pack_dict(obj: serde_json::value::Value) -> Vec<u8> {
    let dict = obj.as_object().unwrap();
    let mut packed: Vec<u8> = b"\x04".to_vec();
    
    let num = dict.len() as i64;
    packed.append( &mut num.to_ne_bytes().to_vec());
    
    for (k,v) in dict{
        let mut packed_key = pack_blob(serde_json::Value::String(k.clone()));
        packed.append(&mut len_u64(packed_key.clone()));
        packed.append( &mut packed_key);

        let mut packed_val = pack_blob(v.clone());
        packed.append(&mut len_u64(packed_val.clone()));
        packed.append( &mut packed_val);
    }

    return packed;
}

fn unpack_dict(mut bytes:Vec<u8>) -> HashMap<String, serde_json::value::Value> {
    let mut len_dict: Vec<u8> = bytes;
    bytes = len_dict.split_off(8);

    let mut dict = HashMap::new();

    for n in 0..*len_dict.get(0).unwrap(){
        let mut len: Vec<u8> = bytes;
        bytes = len.split_off(8);
        
        let rest = bytes.split_off((*len.get(0).unwrap()).into());
        let key = read_blob(bytes).as_str().unwrap().to_string();
        bytes = rest;
    
        let mut len: Vec<u8> = bytes;
        bytes = len.split_off(8);
        
        let rest = bytes.split_off((*len.get(0).unwrap()).into());
        let val = read_blob(bytes);
        bytes = rest;

        dict.insert(key, val);
    }

    dict
}

fn pack_vec(obj: serde_json::value::Value) -> Vec<u8> {
    let arr = obj.as_array().unwrap();

    let mut packed: Vec<u8> = b"\x02".to_vec();
    
    let num = arr.len() as i64;
    packed.append( &mut num.to_ne_bytes().to_vec());
    
    for n in 0..arr.len() {
        println!("{:?}", arr.get(n).unwrap());
        let packed_data: &mut Vec<u8> = &mut pack_blob(arr.get(n).unwrap().clone());
        packed.append(&mut len_u64(packed_data.clone()));
        packed.append(packed_data);
    }
    return packed;
}


fn unpack_vec(mut bytes:Vec<u8>) -> Vec<serde_json::value::Value>{
    let mut len_list: Vec<u8> = bytes;
    bytes = len_list.split_off(8);

    let mut vec = Vec::<serde_json::value::Value>::new();

    for _ in 0..*len_list.get(0).unwrap(){
        let mut len: Vec<u8> = bytes;
        bytes = len.split_off(8);
        
        let rest = bytes.split_off((*len.get(0).unwrap()).into());
        vec.push(read_blob(bytes));
        bytes = rest;
    }

    vec
}
          
// macro_rules! pack_set {
//     ($ty:ty) => {
//         impl Pack for HashSet<$ty> {
//             fn pack(&self) -> Vec<u8> {
//                 let mut packed: Vec<u8> = b"\x03".to_vec();
//                 let num = self.len() as i64;
//                 packed.append( &mut num.to_ne_bytes().to_vec());
                
//                 for n in self{
//                     let mut packed_data = pack_blob(self.get(n).unwrap().clone());
//                     packed.append(&mut len_u64(packed_data.clone()));
//                     packed.append(&mut packed_data);
//                 }

//                 return packed;
//             }

//             fn as_int(self) -> i64 {panic!()}
//             fn as_bool(self) -> bool {panic!()}
//             fn as_float(self) -> f64 {panic!()}
//         }
//     }
// }

// //SET IMPLEMENTATIONS
// pack_set!(i64);
// pack_set!(String);
// pack_set!(&str);
// pack_set!(bool);
// //pack_set!(f64); // floats not compatible with hashset

// // Work in progress
// fn unpack_set(mut bytes:Vec<u8>) -> HashSet<Value>{
//     let mut len_set: Vec<u8> = bytes;
//     bytes = len_set.split_off(8);
//     let mut set = HashSet::<Value>::new();

//     for _ in 0..*len_set.get(0).unwrap(){
//         let mut len: Vec<u8> = bytes;
//         bytes = len.split_off(8);
        
//         let rest = bytes.split_off((*len.get(0).unwrap()).into());
//         read_blob(bytes);
//         bytes = rest;
//     }

//     println!("{:?}", set);
//     set
// }


// ----- PRIMITIVE DATA TYPE IMPLEMENTATION ----- //

pub trait Pack {
    fn pack(&self) -> Vec<u8>;
}

macro_rules! pack_string {
    ($ty:ty) => {
        impl Pack for $ty{
            fn pack(&self) -> Vec<u8>{
                let mut packed: Vec<u8> = b"\x05".to_vec();
            
                let n_bytes = self.as_bytes().len() as i64;
                packed.append( &mut n_bytes.to_ne_bytes().to_vec());

                packed.append(&mut self.as_bytes().to_vec()); // Data
            
                return packed;
            }
        }
    }
}

pack_string!(&str);
pack_string!(String);

fn unpack_string(mut bytes:Vec<u8>) -> String {
    // Get n_byte
    let pos = 8;
    let mut n_bytes: Vec<u8> = bytes;
    bytes = n_bytes.split_off(pos);
    
    String::from_utf8(bytes).unwrap()
}

impl Pack for i64 {
    fn pack(&self) -> Vec<u8> {
        let mut packed: Vec<u8> = b"\x0a".to_vec(); // Prefix
        let mut data: Vec<u8> = self.to_ne_bytes().to_vec(); // Data

        // Get size
        let mut n_bytes = self.to_ne_bytes().len(); // Size
        while n_bytes > 0 {
            if *data.get(n_bytes - 1).unwrap() == 0 {
                n_bytes = n_bytes - 1;
            }
            else {
                break;
            }
        }

        // Get rid of extra 0's
        data.truncate(n_bytes);

        packed.push(n_bytes as u8);
        packed.push(b'\0');

        packed.append(&mut data); // Data
        
        return packed;
    }
}

fn unpack_int(mut bytes:Vec<u8>) -> i64{
    // Get n_byte
    let pos = get_zero_terminated_pos(&bytes);
    bytes.remove(pos);
    let mut n_bytes: Vec<u8> = bytes;
    bytes = n_bytes.split_off(pos);

    let mut i = 8 - *n_bytes.get(0).unwrap();
    while i > 0 {
        bytes.push(b'\0'); 
        i = i - 1;
    }

    let byte_arr = bytes.try_into().unwrap();
    i64::from_ne_bytes(byte_arr)
}

impl Pack for bool {
    fn pack(&self) -> Vec<u8> {
        let mut packed: Vec<u8> = b"\x0b".to_vec(); // Prefix
        packed.push(*self as u8);
        return packed;
    }
}

fn unpack_bool(bytes:Vec<u8>) -> bool {

    *bytes.get(0).unwrap() != 0
}

impl Pack for f64 {
    fn pack(&self) -> Vec<u8> {
        let mut packed: Vec<u8> = b"\x0d".to_vec(); // Prefix
        let mut data: Vec<u8> = self.to_ne_bytes().to_vec(); // Data
        
        packed.append(&mut data);

        return packed;
    }
}

fn unpack_float(bytes:Vec<u8>) -> f64{
    let byte_arr = bytes.try_into().unwrap();
    f64::from_ne_bytes(byte_arr)
}

fn get_zero_terminated_pos (blob: &Vec<u8>) -> usize{
    let mut iter = blob.iter();
    return iter.position(|x| x == &b'\0').unwrap();
}

fn len_u64 (bytes: Vec<u8>) -> Vec<u8> {
    let num = bytes.len() as i64;
    num.to_ne_bytes().to_vec()
}