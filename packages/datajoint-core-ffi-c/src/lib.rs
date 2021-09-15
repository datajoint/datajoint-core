
extern crate datajoint_core;

#[no_mangle]
pub extern "C" fn conn() {
    datajoint_core::connection::conn();
}
