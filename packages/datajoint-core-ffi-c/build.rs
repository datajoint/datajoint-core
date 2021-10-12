extern crate cbindgen;

use std::env;
use cbindgen::Language;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut parse_config = cbindgen::ParseConfig::default();
    parse_config.parse_deps = true;
    parse_config.include = Some(vec![String::from("datajoint-core")]);

    let mut enum_config = cbindgen::EnumConfig::default();
    enum_config.prefix_with_name = true;

    let mut config = cbindgen::Config::default();
    config.enumeration = enum_config;
    config.parse = parse_config;
    config.no_includes = true;
    config.language = Language::C;

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("datajoint-core-ffi-c.h");
}