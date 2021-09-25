extern crate cbindgen;

use std::env;
use cbindgen::Language;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(Language::C)
        .with_no_includes()
        .with_parse_include(&["datajoint-core"])
        .with_parse_deps(true)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("datajoint-core-ffi-c.h");
}