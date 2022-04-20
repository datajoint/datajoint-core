use datajoint_core::{
    blob::Blob,
};
use serde_json::json;

#[test]
fn test_blob() {
    let item = json!({
        "key1": true,
        "key2": ["val", "val", "val"],
        "key3": { "keyX": 12_i64 }
    });
    let _x = "hello";
    let _v = json!({ "an": "object" });
    let var = Blob::pack(item);
    println!("{:02x?}", var);
    println!("{}\n", Blob::unpack(var));

    // let response = serde_json::to_string(&item).unwrap();
}
