use std::convert::TryInto;
use std::collections::HashMap;


fn main() {
    let vec = vec![1, 2, 3];
    let test = HashMap::from([
        ("spikes",vec),
    ]);

    //let hello = "hello world";

    // let test1 = HashMap::from([
    //     (1,10),
    //     (2,20),
    //     (3,30),
    //     (4,40),
    //     (5,50),
    // ]);
    
    let var = Blob::pack(test);
    //println!("{:02X?}", var); //hex
    println!("{:?}", var);

    Blob::unpack(var);
}

pub struct Blob {
    pub packed: Vec<u8>,
}

pub struct UnBlob<T>{
    pub result: T,
}


impl Blob {
    pub fn unpack (mut blob: Vec<u8>){
        // Get Protocol
        let pos = get_zero_terminated_pos(&blob);
        blob.remove(pos);
        let mut protocol: Vec<u8> = blob;
        blob = protocol.split_off(pos);
        //println!("Protocol: {:?} Blob: {:?}", protocol, blob);
        read_blob(blob);
    }

    pub fn pack<T: Pack>(obj: T) -> Vec<u8> {
        let mut blob: Vec<u8> = b"dj0\0".to_vec();
        blob.append(&mut pack_blob(obj));
        return blob;
    }
}


fn read_blob(mut blob: Vec<u8>){
    // Get Prefix
    let prefix = blob.get(0).cloned().unwrap();
    blob = blob.split_off(1);
    //println!("Prefix: {} Blob: {:?}", prefix, blob);

    match prefix{
        b'\x02'=> unpack_list(blob),
        b'\x04'=> unpack_dictionary(blob),
        b'\x05'=>println!("{}", unpack_string(blob)),
        b'\x0a'=>println!("{}", unpack_int(blob)),
        _=>println!("Not Implemented")
    }
}



#[inline]
fn pack_blob<T: Pack>(obj: T) -> Vec<u8> {
    let type_var = check_type(&obj);

    let packed_data: Vec<u8> = {
        match type_var {
            "i32" => obj.as_int().pack(),
            "i64" => obj.as_int().pack(),
            _ => obj.pack(), // List, Dictionary, String
        }
    };

    return packed_data;
}


pub trait Pack {
    //REMEMBER TO ADD YOUR FUNCTION HERE WHEN YOU WORK ON IT AND TO INCLUDE ALL
    fn pack(&self) -> Vec<u8>;

    fn as_string(self) -> String;
    fn as_int(self) -> i64;
}

macro_rules! pack_list {
    ($ty:ty) => {
        impl Pack for Vec<$ty> {
            fn pack(&self) -> Vec<u8> {
                let mut packed: Vec<u8> = b"\x02".to_vec();
                
                let num = self.len() as i64;
                packed.append( &mut num.to_ne_bytes().to_vec());
                
                for n in 0..self.len() {
                    let mut packed_data = pack_blob(self.get(n).unwrap().clone());
                    packed.append(&mut len_u64(packed_data.clone()));
                    packed.append( &mut packed_data);
                }

                return packed;
            }

            fn as_string(self) -> String {panic!()}
            fn as_int(self) -> i64 {panic!()}
        }
    }
}

//LIST IMPLEMENTATIONS
pack_list!(i64);
pack_list!(String);
pack_list!(&str);

//Probably have to think of other data types within the list too instead of just returning int
//Something like ["hello", "world"] would also need to be taken account of
fn unpack_list(mut bytes:Vec<u8>){
    //let result: Vec<T> = Vec::New();
    let mut len_list: Vec<u8> = bytes;
    bytes = len_list.split_off(8);

    for _ in 0..*len_list.get(0).unwrap() {
        let mut len: Vec<u8> = bytes;
        bytes = len.split_off(8);
        
        let rest = bytes.split_off((*len.get(0).unwrap()).into());
        //result.push(read_blob(bytes));
        read_blob(bytes);
        bytes = rest;
    }
}

macro_rules! pack_dictionary {
    ($key:ty, $val:ty) => {
        impl Pack for HashMap<$key, $val> {
            fn pack(&self) -> Vec<u8> {
                let mut packed: Vec<u8> = b"\x04".to_vec();
                
                let num = self.len() as i64;
                packed.append( &mut num.to_ne_bytes().to_vec());
                
                for (k,v) in self{
                    let mut packed_key = pack_blob(k.clone());
                    packed.append(&mut len_u64(packed_key.clone()));
                    packed.append( &mut packed_key);

                    let mut packed_val = pack_blob(v.clone());
                    packed.append(&mut len_u64(packed_val.clone()));
                    packed.append( &mut packed_val);
                }

                return packed;
            }

            fn as_string(self) -> String {panic!()}
            fn as_int(self) -> i64 {panic!()}
        }
    }
}

// DICTIONARY IMPLEMENTATIONS
macro_rules! permutations {
    ($ty:ty) => {
        pack_dictionary!(i64, $ty);
        pack_dictionary!($ty, Vec<i64>);
        pack_dictionary!(String, $ty);
        pack_dictionary!($ty, Vec<String>);
        pack_dictionary!($ty, Vec<&str>);
        pack_dictionary!(&str, $ty);

    }
}
permutations!(i64);
permutations!(String);
permutations!(&str);

fn unpack_dictionary(mut bytes:Vec<u8>){
    let mut len_dict: Vec<u8> = bytes;
    bytes = len_dict.split_off(8);

    for _ in 0..*len_dict.get(0).unwrap()*2{
        let mut len: Vec<u8> = bytes;
        bytes = len.split_off(8);
        
        let rest = bytes.split_off((*len.get(0).unwrap()).into());
        read_blob(bytes);
        bytes = rest;
    }
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

            #[inline]
            fn as_string(self) -> String {String::from(self)}
            fn as_int(self) -> i64 {panic!()}
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

    #[inline]
    fn as_string(self) -> String {panic!()}
    fn as_int(self) -> i64 {self}
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
    let x = i64::from_ne_bytes(byte_arr);
    let unBlob = UnBlob{
        result: x,
    };
    i64::from_ne_bytes(byte_arr)
}

fn check_type<T>(_obj: &T) -> &str {
    let type_var: &str = std::any::type_name::<T>();
    //println!("{}", std::any::type_name::<T>());
    return type_var;
}

fn get_zero_terminated_pos (blob: &Vec<u8>) -> usize{
    let mut iter = blob.iter();
    return iter.position(|x| x == &b'\0').unwrap();
}

fn len_u64 (bytes: Vec<u8>) -> Vec<u8> {
    let num = bytes.len() as i64;
    num.to_ne_bytes().to_vec()
}

// from datajoint.blob import pack, unpack
// payload = 2147483647
// packed_payload = pack(payload)
// print([p for p in packed_payload])