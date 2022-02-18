use std::convert::TryInto;

fn main() {
    let mut var: Vec<u8>= b"dj0\0".to_vec();
    //var.append( &mut pack_int(5));
    var.append(&mut pack_string("test"));


    //let bytes:Vec<u8> = vec![5, 0, 0, 0];
    //let  unpackedvar = unpack_int(bytes);

    println!("{:?}", var);
    //println!("{}", unpackedvar);
}

fn pack_string(x: &str) -> Vec<u8>{
    let mut packed: Vec<u8> = b"\x0a".to_vec();

    let n_bytes = x.as_bytes().len();
    packed.push(n_bytes.try_into().unwrap());
    packed.push(b'\0');

    packed.append(&mut x.as_bytes().to_vec()); // Data

    return packed;
      


}
// def pack_string(s):
// blob = s.encode()
// return b"\5" + len_u64(blob) + 


fn pack_int(x: i32) -> Vec<u8>{
    let mut packed: Vec<u8> = b"\x0a".to_vec(); // Prefix

    let n_bytes = x.to_ne_bytes().len() as u8; // Size
    packed.push(n_bytes);
    packed.push(b'\0');

    packed.append(&mut x.to_ne_bytes().to_vec()); // Data
    
    return packed;
}

fn unpack_int(bytes:Vec<u8>) -> i32
{

    let byte_arr = bytes.try_into().unwrap_or_else(|bytes: Vec<u8>|panic!("ERROR: Unable to convert to array"));
    let num = i32::from_ne_bytes(byte_arr);

    return num;
}

