<<<<<<< cffi-settings
extern crate datajoint_core;
use libc::{c_char};
use std::ffi::{CString, CStr};
use datajoint_core::connection::{ConnectionSettings, DatabaseType};

#[no_mangle]
pub extern "C" fn connection_settings_new() -> *mut ConnectionSettings{
    Box::into_raw(Box::new(ConnectionSettings::new()))
}
#[no_mangle]
pub extern "C" fn connection_settings_free(ptr: *mut ConnectionSettings){
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

// Getting a warning saying having it as databasetype is "Not FFI Safe"
#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_database_type(ptr: *mut ConnectionSettings, dbtype: DatabaseType){  
    let connection: &mut ConnectionSettings =  {
        if ptr.is_null(){
            return
        }
        &mut *ptr
    };

    connection.database_type = dbtype;
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_username(ptr: *mut ConnectionSettings, username: *const c_char){
    let connection: &mut ConnectionSettings =  {
        if ptr.is_null(){
            return
        }
        &mut *ptr
    };

    let user =  {
        if username.is_null() {
           return  
        }
        CStr::from_ptr(username).to_string_lossy().to_owned()
    };
    connection.username = user.to_string();
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_password(ptr: *mut ConnectionSettings, password: *const c_char){
    let connection: &mut ConnectionSettings = {
        if ptr.is_null(){
            return
        }
        &mut *ptr
    };

    let pass = {
        if password.is_null() {
            return 
        }
        CStr::from_ptr(password).to_string_lossy().to_owned()
    };
    connection.password = pass.to_string();
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_hostname(ptr: *mut ConnectionSettings, hostname: *const c_char){
    let connection: &mut ConnectionSettings = {
        if ptr.is_null(){
            return
        }
        &mut *ptr
    };

    let host = {
        if hostname.is_null() {

        }
        CStr::from_ptr(hostname).to_string_lossy().to_owned()
    };
    connection.hostname = host.to_string();
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_port(ptr: *mut ConnectionSettings, port: u16){
    
    let connection: &mut ConnectionSettings = {
        if ptr.is_null(){
            return
        }
        &mut *ptr
    };
    connection.port = port;
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_databae_name(ptr: *mut ConnectionSettings, database_name: *const c_char){
    let connection: &mut ConnectionSettings = {
        if ptr.is_null(){
            return
        }
        &mut *ptr
    };

    let database = {
        if database_name.is_null() {

        }
        CStr::from_ptr(database_name).to_string_lossy().to_owned()
    };
    connection.database_name = database.to_string();
}

// Could not figure out how to make this return just connection.database_type. Nothing seemed to work
#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_database_type(ptr: *mut ConnectionSettings) -> *mut DatabaseType{
    
    let connection: &mut ConnectionSettings = {
        if ptr.is_null(){
            // What to return?
        }
        &mut *ptr
    };
    
    let databasetype = &mut connection.database_type;
    databasetype
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_username(ptr: *const ConnectionSettings) -> *mut c_char{
    let connection: &ConnectionSettings = {
        if ptr.is_null(){
            // What to return?
        }
        &*ptr
    };

    let str_bytes = connection.username.as_bytes();
    CString::new(str_bytes).unwrap().into_raw()
}


#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_password(ptr: *const ConnectionSettings) -> *mut c_char{
    let connection: &ConnectionSettings = {
        if ptr.is_null(){
            // What to return?
        }
        &*ptr
    };

    let str_bytes = connection.password.as_bytes();
    CString::new(str_bytes).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_hostname(ptr: *const ConnectionSettings) -> *mut c_char{
    let connection: &ConnectionSettings = {
        if ptr.is_null(){
            // What to return?
        }
        &*ptr
    };

    let str_bytes = connection.hostname.as_bytes();
    CString::new(str_bytes).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_port(ptr: *const ConnectionSettings) -> u16{
    let connection: &ConnectionSettings = {
        if ptr.is_null(){
            return 0
        }
        &*ptr
    };

    connection.port
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_database_name(ptr: *const ConnectionSettings) -> *mut c_char{
    let connection: &ConnectionSettings = {
        if ptr.is_null(){
            // What to return?
        }
        &*ptr
    };

    let str_bytes = connection.database_name.as_bytes();
    CString::new(str_bytes).unwrap().into_raw()
}

// From http://jakegoulding.com/rust-ffi-omnibus/string_return/ (Only way we could think about doing it without making a copy)
#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_string_free(s: *mut c_char){
    if s.is_null(){
        return
    }
    CString::from_raw(s);
}

// Not sure if any "free" function needs to be added for database_type or u16