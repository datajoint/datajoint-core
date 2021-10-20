use crate::util::OptionalBool;
use datajoint_core::connection::{ConnectionSettings, DatabaseType};
use libc::c_char;
use std::ffi::{CStr, CString};
use std::ptr;

#[no_mangle]
pub extern "C" fn connection_settings_new() -> *mut ConnectionSettings {
    Box::into_raw(Box::new(ConnectionSettings::new()))
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_free(this: *mut ConnectionSettings) {
    if !this.is_null() {
        Box::from_raw(this);
    }
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_database_type(
    this: *mut ConnectionSettings,
    dbtype: DatabaseType,
) {
    if this.is_null() {
        return;
    }
    let connection: &mut ConnectionSettings = { &mut *this };

    connection.database_type = dbtype;
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_username(
    this: *mut ConnectionSettings,
    username: *const c_char,
) {
    if this.is_null() || username.is_null() {
        return;
    }
    let connection: &mut ConnectionSettings = { &mut *this };

    let user = CStr::from_ptr(username).to_string_lossy().to_owned();
    connection.username = user.to_string();
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_password(
    this: *mut ConnectionSettings,
    password: *const c_char,
) {
    if this.is_null() || password.is_null() {
        return;
    }
    let connection: &mut ConnectionSettings = { &mut *this };

    let pass = CStr::from_ptr(password).to_string_lossy().to_owned();
    connection.password = pass.to_string();
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_hostname(
    this: *mut ConnectionSettings,
    hostname: *const c_char,
) {
    if this.is_null() || hostname.is_null() {
        return;
    }
    let connection: &mut ConnectionSettings = { &mut *this };

    let host = CStr::from_ptr(hostname).to_string_lossy().to_owned();
    connection.hostname = host.to_string();
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_port(this: *mut ConnectionSettings, port: u16) {
    if this.is_null() {
        return;
    }
    let connection: &mut ConnectionSettings = { &mut *this };
    connection.port = port;
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_database_name(
    this: *mut ConnectionSettings,
    database_name: *const c_char,
) {
    if this.is_null() || database_name.is_null() {
        return;
    }
    let connection: &mut ConnectionSettings = { &mut *this };

    let database = CStr::from_ptr(database_name).to_string_lossy().to_owned();
    connection.database_name = database.to_string();
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_use_tls(
    this: *mut ConnectionSettings,
    tls_ssl: OptionalBool,
) {
    if this.is_null() {
        return;
    }
    let connection: &mut ConnectionSettings = { &mut *this };
    match tls_ssl {
        OptionalBool::True => connection.use_tls = Some(true),
        OptionalBool::False => connection.use_tls = Some(false),
        OptionalBool::None => connection.use_tls = None,
    }
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_database_type(
    this: *mut ConnectionSettings,
) -> DatabaseType {
    if this.is_null() {
        // TODO: Return a different default value?
        return DatabaseType::MySql;
    }
    let connection: &ConnectionSettings = { &*this };

    connection.database_type
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_username(
    this: *const ConnectionSettings,
) -> *const c_char {
    if this.is_null() {
        return ptr::null();
    }
    let connection: &ConnectionSettings = { &*this };

    let str_bytes = connection.username.as_bytes();
    match CString::new(str_bytes) {
        Err(_) => std::ptr::null(),
        Ok(str_bytes) => str_bytes.into_raw(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_password(
    this: *const ConnectionSettings,
) -> *const c_char {
    if this.is_null() {
        return ptr::null();
    }
    let connection: &ConnectionSettings = { &*this };

    let str_bytes = connection.password.as_bytes();
    match CString::new(str_bytes) {
        Err(_) => std::ptr::null(),
        Ok(str_bytes) => str_bytes.into_raw(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_hostname(
    this: *const ConnectionSettings,
) -> *const c_char {
    if this.is_null() {
        return ptr::null();
    }
    let connection: &ConnectionSettings = { &*this };

    let str_bytes = connection.hostname.as_bytes();
    match CString::new(str_bytes) {
        Err(_) => std::ptr::null(),
        Ok(str_bytes) => str_bytes.into_raw(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_port(this: *const ConnectionSettings) -> u16 {
    if this.is_null() {
        return 0;
    }
    let connection: &ConnectionSettings = { &*this };

    connection.port
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_database_name(
    this: *const ConnectionSettings,
) -> *const c_char {
    if this.is_null() {
        return ptr::null();
    }
    let connection: &ConnectionSettings = { &*this };

    let str_bytes = connection.database_name.as_bytes();
    match CString::new(str_bytes) {
        Err(_) => std::ptr::null(),
        Ok(str_bytes) => str_bytes.into_raw(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_use_tls(
    this: *const ConnectionSettings,
) -> OptionalBool {
    if this.is_null() {
        return OptionalBool::None;
    }
    let connection: &ConnectionSettings = { &*this };

    match connection.use_tls {
        Some(true) => OptionalBool::True,
        Some(false) => OptionalBool::False,
        None => OptionalBool::None,
    }
}
