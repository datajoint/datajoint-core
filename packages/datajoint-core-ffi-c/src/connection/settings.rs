<<<<<<< cffi-settings
extern crate datajoint_core;
use libc::{c_char};
use std::ffi::{CString, CStr};
use std::ptr;
use datajoint_core::connection::{ConnectionSettings, DatabaseType};

#[no_mangle]
pub extern "C" fn connection_settings_new() -> *mut ConnectionSettings {
    Box::into_raw(Box::new(ConnectionSettings::new()))
}
#[no_mangle]
pub extern "C" fn connection_settings_free(pointers: *mut ConnectionSettings) {
    if pointers.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(pointers);
    }
}

// Getting a warning saying having it as databasetype is "Not FFI Safe"
#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_database_type(pointers: *mut ConnectionSettings, dbtype: DatabaseType) {  
    let connection: &mut ConnectionSettings = {
        if pointers.is_null() {
            return
        }
        &mut *pointers
    };

    connection.database_type = dbtype;
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_username(pointers: *mut ConnectionSettings, username: *const c_char) {
    let connection: &mut ConnectionSettings = {
        if pointers.is_null() || username.is_null() {
            return
        }
        &mut *pointers
    };

    let user = {
        if username.is_null() {
           return  
        }
        CStr::from_ptr(username).to_string_lossy().to_owned()
    };
    connection.username = user.to_string();
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_password(pointers: *mut ConnectionSettings, password: *const c_char) {
    let connection: &mut ConnectionSettings = {
        if pointers.is_null() || password.is_null(){
            return
        }
        &mut *pointers
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
pub unsafe extern "C" fn connection_settings_set_hostname(pointers: *mut ConnectionSettings, hostname: *const c_char) {
    let connection: &mut ConnectionSettings = {
        if pointers.is_null() || hostname.is_null() {
            return
        }
        &mut *pointers
    };

    let host = {
        if hostname.is_null() {

        }
        CStr::from_ptr(hostname).to_string_lossy().to_owned()
    };
    connection.hostname = host.to_string();
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_port(pointers: *mut ConnectionSettings, port: u16) {
    
    let connection: &mut ConnectionSettings = {
        if pointers.is_null() {
            return
        }
        &mut *pointers
    };
    connection.port = port;
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_databae_name(pointers: *mut ConnectionSettings, database_name: *const c_char) {
    let connection: &mut ConnectionSettings = {
        if pointers.is_null() || database_name.is_null(){
            return
        }
        &mut *pointers
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
pub unsafe extern "C" fn connection_settings_get_database_type(pointers: *mut ConnectionSettings) -> DatabaseType {
    
    let connection: &mut ConnectionSettings = {
        if pointers.is_null() {
            // Not sure what to return, can't return a pointer
        }
        &mut *pointers
    };
    
    //let databasetype = &mut connection.database_type;
    //databasetype
    connection.database_type
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_username(pointers: *const ConnectionSettings) -> *mut c_char {
    let connection: &ConnectionSettings = {
        if pointers.is_null() {
            return ptr::null_mut()
        }
        &*pointers
    };

    let str_bytes = connection.username.as_bytes();
    CString::new(str_bytes).unwrap().into_raw()
}


#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_password(pointers: *const ConnectionSettings) -> *mut c_char {
    let connection: &ConnectionSettings = {
        if pointers.is_null() { 
            return ptr::null_mut()
        }
        &*pointers
    };

    let str_bytes = connection.password.as_bytes();
    CString::new(str_bytes).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_hostname(pointers: *const ConnectionSettings) -> *mut c_char {
    let connection: &ConnectionSettings = {
        if pointers.is_null() {
            return ptr::null_mut()  
        }
        &*pointers
    };

    let str_bytes = connection.hostname.as_bytes();
    CString::new(str_bytes).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_port(pointers: *const ConnectionSettings) -> u16 {
    let connection: &ConnectionSettings = {
        if pointers.is_null() {
            return 0
        }
        &*pointers
    };

    connection.port
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_database_name(pointers: *const ConnectionSettings) -> *mut c_char {
    let connection: &ConnectionSettings = {
        if pointers.is_null() {
            return ptr::null_mut()    
        }
        &*pointers
    };

    let str_bytes = connection.database_name.as_bytes();
    CString::new(str_bytes).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_string_free(s: *mut c_char) {
    if s.is_null() {
        return
    }
    CString::from_raw(s);
}

// Not sure if any "free" function needs to be added for database_type or u16