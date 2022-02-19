use std::convert::TryInto;

fn main() {
    let var = pack(2147483647_i32);
    println!("{:?}", var);

    unpack(var);
}

fn unpack (mut blob: Vec<u8>){
    // Get Protocol
    let pos = get_zero_terminated_pos(&blob);
    blob.remove(pos);
    let mut protocol: Vec<u8> = blob;
    blob = protocol.split_off(pos);
    println!("Protocol: {:?} Blob: {:?}", protocol, blob);

    read_blob(blob);
}

fn read_blob (mut blob: Vec<u8>){
    // Get Prefix
    let prefix = blob.get(0).cloned().unwrap();
    blob = blob.split_off(1);
    println!("Prefix: {} Blob: {:?}", prefix, blob);

    match prefix{
        b'\x0a'=>println!("{}", unpack_int(blob)),
        //b'\x0a'=>return unpack_int(blob),
        _=>println!("default")
    }

}

#[inline]
fn pack<T: Pack>(obj: T) -> Vec<u8> {
    let protocol: Vec<u8> = b"dj0\0".to_vec();

    let type_var = check_type(&obj);
    let mut packed_data: Vec<u8> = {
        match type_var {
            "i32" => obj.as_i32().pack(),
            _ => panic!(),
        }
    };

    let mut blob = protocol;
    blob.append(&mut packed_data);
    return blob;
}

trait Pack {
    fn pack(&self) -> Vec<u8>;

    fn as_string(self) -> String;
    fn as_i32(self) -> i32;
}
    
impl Pack for i32 {
    #[inline]
    fn as_string(self) -> String {panic!()}
    fn as_i32(self) -> i32 {self}

    fn pack(&self) -> Vec<u8> {
        let mut packed: Vec<u8> = b"\x0a".to_vec(); // Prefix

        let n_bytes = self.to_ne_bytes().len() as u8; // Size
        packed.push(n_bytes);
        packed.push(b'\0');

        packed.append(&mut self.to_ne_bytes().to_vec()); // Data
        
        return packed;
    }
}

fn unpack_int(mut bytes:Vec<u8>) -> i32{
    // Get n_bytes
    let pos = get_zero_terminated_pos(&bytes);
    bytes.remove(pos);
    let mut n_bytes: Vec<u8> = bytes;
    bytes = n_bytes.split_off(pos);

    let byte_arr = bytes.try_into().unwrap();
    let num = i32::from_ne_bytes(byte_arr);
    return num;
}

fn check_type<T>(_obj: &T) -> &str {
    let type_var: &str = std::any::type_name::<T>();
    println!("{}", std::any::type_name::<T>());

    return type_var;
}

fn get_zero_terminated_pos (blob: &Vec<u8>) -> usize{
    let mut iter = blob.iter();
    return iter.position(|x| x == &b'\0').unwrap();
}

