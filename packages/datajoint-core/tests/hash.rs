use datajoint_core::hash::{Hash};

#[test]
fn test_hash() {
    let mut result = Hash::uuid_from_buffer(b"abc");
    println!("[RESULT]: {:?}", result);
    result = Hash::uuid_from_buffer(b"");
    println!("[RESULT]: {:?}", result);
    
    // result = Hash::uuid_from_file("/Users/thi.vu/datajoint/thi_branch/datajoint-core/packages/datajoint-core/src/hash/test.txt".to_string());
    // println!("[RESULT]: {:?}", result);
}
