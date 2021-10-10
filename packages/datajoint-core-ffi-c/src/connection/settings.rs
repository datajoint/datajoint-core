extern crate datajoint_core;


use libc::c_char;
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

#[no_mangle]
pub extern "C" fn connection_settings_set_database_type(ptr: *mut ConnectionSettings, ptr_enum: *mut DatabaseType) -> i8 {
    let connection: &mut ConnectionSettings = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let enumdatabase_type = unsafe {
        assert!(!ptr_enum.is_null());
        Box::from_raw(ptr_enum)
    };

    connection.database_type = *enumdatabase_type;
    return 0
}

#[no_mangle]
pub extern "C" fn connection_settings_set_username(ptr: *mut ConnectionSettings, username: *const c_char) -> i8{
    let connection: &mut ConnectionSettings = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let user = unsafe {
        if username.is_null() {
            return -1;
        }
        CStr::from_ptr(username).to_string_lossy().to_owned()
    };
    connection.username = user.to_string();
    return 0
}

#[no_mangle]
pub extern "C" fn connection_settings_set_password(ptr: *mut ConnectionSettings, password: *const c_char) -> i8{
    let connection: &mut ConnectionSettings = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let pass = unsafe {
        if password.is_null() {
            return -1;
        }
        CStr::from_ptr(password).to_string_lossy().to_owned()
    };
    connection.password = pass.to_string();

    return 0
}

#[no_mangle]
pub extern "C" fn connection_settings_set_hostname(ptr: *mut ConnectionSettings, hostname: *const c_char) -> i8{
    let connection: &mut ConnectionSettings = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let host = unsafe {
        if hostname.is_null() {
            return -1;
        }
        CStr::from_ptr(hostname).to_string_lossy().to_owned()
    };
    connection.hostname = host.to_string();
    return 0
}

#[no_mangle]
pub extern "C" fn connection_settings_set_port(ptr: *mut ConnectionSettings, port: u16) -> i8{
    
    let connection: &mut ConnectionSettings = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    connection.port = port;
    return 0
}

#[no_mangle]
pub extern "C" fn connection_settings_set_databae_name(ptr: *mut ConnectionSettings, database_name: *const c_char) -> i8{
    let connection: &mut ConnectionSettings = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let database = unsafe {
        if database_name.is_null() {
            return -1;
        }
        CStr::from_ptr(database_name).to_string_lossy().to_owned()
    };
    connection.database_name = database.to_string();
    return 0
}

#[no_mangle]
pub extern "C" fn connection_settings_get_database_type(ptr: *mut ConnectionSettings) -> *mut DatabaseType{
    
    let connection: &mut ConnectionSettings = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    
    let databasetype = &mut connection.database_type;
    databasetype
}

#[no_mangle]
pub extern "C" fn connection_settings_get_username(ptr: *mut ConnectionSettings) -> *mut c_char{
    let connection: &mut ConnectionSettings = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let username = String::from(&connection.username);
    let username = CString::new(username).unwrap();
    username.into_raw()
}

#[no_mangle]
pub extern "C" fn connection_settings_get_password(ptr: *mut ConnectionSettings) -> *mut c_char{
    let connection: &mut ConnectionSettings = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let password = String::from(&connection.password);
    let password = CString::new(password).unwrap();
    password.into_raw()
}

#[no_mangle]
pub extern "C" fn connection_settings_get_hostname(ptr: *mut ConnectionSettings) -> *mut c_char{
    let connection: &mut ConnectionSettings = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let hostname = String::from(&connection.password);
    let hostname = CString::new(hostname).unwrap();
    hostname.into_raw()
}

#[no_mangle]
pub extern "C" fn connection_settings_get_port(ptr: *mut ConnectionSettings) -> u16{
    let connection: &mut ConnectionSettings = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let port = connection.port;
    port
}

#[no_mangle]
pub extern "C" fn connection_settings_get_database_name(ptr: *mut ConnectionSettings) -> *mut c_char{
    let connection: &mut ConnectionSettings = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let database_name = String::from(&connection.password);
    let database_name = CString::new(database_name).unwrap();
    database_name.into_raw()
}