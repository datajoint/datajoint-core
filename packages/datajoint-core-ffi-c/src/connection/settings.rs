use crate::error::datajoint_core_set_last_error;
use crate::util::OptionalBool;
use datajoint_core::common::DatabaseType;
use datajoint_core::connection::ConnectionSettings;
use datajoint_core::error::{DataJointError, ErrorCode};
use datajoint_core::util::IntegerEnum;
use libc::c_char;
use std::ffi::{CStr, CString};
use std::ptr;

/// Creates a new settings object.
#[no_mangle]
pub extern "C" fn connection_settings_new() -> *mut ConnectionSettings {
    Box::into_raw(Box::new(ConnectionSettings::new()))
}

/// Frees a settings object.
#[no_mangle]
pub unsafe extern "C" fn connection_settings_free(this: *mut ConnectionSettings) {
    if !this.is_null() {
        Box::from_raw(this);
    }
}

/// Sets the database type, which represents the SQL flavor to use for the connection.
#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_database_type(
    this: *mut ConnectionSettings,
    dbtype: DatabaseType,
) -> i32 {
    if this.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    } else if DatabaseType::from_int(dbtype as i32) == None {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::BadPrimitiveEnumValue))
            as i32;
    }
    let settings: &mut ConnectionSettings = { &mut *this };

    settings.database_type = dbtype;
    ErrorCode::Success as i32
}

/// Sets the username for a database connection.
#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_username(
    this: *mut ConnectionSettings,
    username: *const c_char,
) -> i32 {
    if this.is_null() || username.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }
    let settings: &mut ConnectionSettings = { &mut *this };

    settings.username = match CStr::from_ptr(username).to_str() {
        Err(_) => {
            return datajoint_core_set_last_error(DataJointError::new(ErrorCode::InvalidUtf8String))
                as i32
        }
        Ok(cstr) => cstr.to_string(),
    };
    ErrorCode::Success as i32
}

/// Sets the password for a database connection.
#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_password(
    this: *mut ConnectionSettings,
    password: *const c_char,
) -> i32 {
    if this.is_null() || password.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }
    let settings: &mut ConnectionSettings = { &mut *this };

    settings.password = match CStr::from_ptr(password).to_str() {
        Err(_) => {
            return datajoint_core_set_last_error(DataJointError::new(ErrorCode::InvalidUtf8String))
                as i32
        }
        Ok(cstr) => cstr.to_string(),
    };
    ErrorCode::Success as i32
}

/// Sets the hostname for a database connection.
#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_hostname(
    this: *mut ConnectionSettings,
    hostname: *const c_char,
) -> i32 {
    if this.is_null() || hostname.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }
    let settings: &mut ConnectionSettings = { &mut *this };

    settings.hostname = match CStr::from_ptr(hostname).to_str() {
        Err(_) => {
            return datajoint_core_set_last_error(DataJointError::new(ErrorCode::InvalidUtf8String))
                as i32
        }
        Ok(cstr) => cstr.to_string(),
    };
    ErrorCode::Success as i32
}

/// Sets the port for a database connection.
#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_port(
    this: *mut ConnectionSettings,
    port: u16,
) -> i32 {
    if this.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }
    let settings: &mut ConnectionSettings = { &mut *this };
    settings.port = port;
    ErrorCode::Success as i32
}

/// Sets the database name for a database connection.
#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_database_name(
    this: *mut ConnectionSettings,
    database_name: *const c_char,
) -> i32 {
    if this.is_null() || database_name.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }
    let settings: &mut ConnectionSettings = { &mut *this };

    settings.database_name = match CStr::from_ptr(database_name).to_str() {
        Err(_) => {
            return datajoint_core_set_last_error(DataJointError::new(ErrorCode::InvalidUtf8String))
                as i32
        }
        Ok(cstr) => cstr.to_string(),
    };
    ErrorCode::Success as i32
}

/// Specifies how a connection should use TLS.
///
/// Receives an [`OptionalBool`], which represents three-state logic.
/// - [`OptionalBool::True`] - Enforce TLS.
/// - [`OptionalBool::False`] - Do not use TLS.
/// - [`OptionalBool::None`] - Let database decide.
#[no_mangle]
pub unsafe extern "C" fn connection_settings_set_use_tls(
    this: *mut ConnectionSettings,
    use_tls: OptionalBool,
) -> i32 {
    if this.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    } else if OptionalBool::from_int(use_tls as i32) == None {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::BadPrimitiveEnumValue))
            as i32;
    }
    let settings: &mut ConnectionSettings = { &mut *this };
    settings.use_tls = use_tls.into();
    ErrorCode::Success as i32
}

/// Gets the database type entry on the settings object.
#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_database_type(
    this: *mut ConnectionSettings,
) -> DatabaseType {
    if this.is_null() {
        // Just return a default value since there is no way of representing the
        // absence of a value with an enum.
        return DatabaseType::MySql;
    }
    let settings: &ConnectionSettings = { &*this };

    settings.database_type
}

/// Gets the username entry on the settings object.
///
/// [`datajoint_core_cstring_free`][crate::util::datajoint_core_cstring_free] must be called
/// on the string returned from this function to avoid memory leaks.
#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_username(
    this: *const ConnectionSettings,
) -> *const c_char {
    if this.is_null() {
        return ptr::null();
    }
    let settings: &ConnectionSettings = { &*this };

    match CString::new(settings.username.as_bytes()) {
        Err(_) => std::ptr::null(),
        Ok(str_bytes) => str_bytes.into_raw(),
    }
}

/// Gets the password entry on the settings object.
///
/// [`datajoint_core_cstring_free`][crate::util::datajoint_core_cstring_free] must be called
/// on the string returned from this function to avoid memory leaks.
#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_password(
    this: *const ConnectionSettings,
) -> *const c_char {
    if this.is_null() {
        return ptr::null();
    }
    let settings: &ConnectionSettings = { &*this };

    match CString::new(settings.password.as_bytes()) {
        Err(_) => std::ptr::null(),
        Ok(str_bytes) => str_bytes.into_raw(),
    }
}

/// Gets the hostname entry on the settings object.
///
/// [`datajoint_core_cstring_free`][crate::util::datajoint_core_cstring_free] must be called
/// on the string returned from this function to avoid memory leaks.
#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_hostname(
    this: *const ConnectionSettings,
) -> *const c_char {
    if this.is_null() {
        return ptr::null();
    }
    let settings: &ConnectionSettings = { &*this };

    match CString::new(settings.hostname.as_bytes()) {
        Err(_) => std::ptr::null(),
        Ok(str_bytes) => str_bytes.into_raw(),
    }
}

/// Gets the port entry on the settings object.
///
/// [`datajoint_core_cstring_free`][crate::util::datajoint_core_cstring_free] must be called
/// on the string returned from this function to avoid memory leaks.
#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_port(this: *const ConnectionSettings) -> u16 {
    if this.is_null() {
        return 0;
    }
    let settings: &ConnectionSettings = { &*this };

    settings.port
}

/// Gets the database name entry on the settings object.
///
/// [`datajoint_core_cstring_free`][crate::util::datajoint_core_cstring_free] must be called
/// on the string returned from this function to avoid memory leaks.
#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_database_name(
    this: *const ConnectionSettings,
) -> *const c_char {
    if this.is_null() {
        return ptr::null();
    }
    let settings: &ConnectionSettings = { &*this };

    match CString::new(settings.database_name.as_bytes()) {
        Err(_) => std::ptr::null(),
        Ok(str_bytes) => str_bytes.into_raw(),
    }
}

/// Gets the TLS setting entry on the settings object.
///
/// [`datajoint_core_cstring_free`][crate::util::datajoint_core_cstring_free] must be called
/// on the string returned from this function to avoid memory leaks.
#[no_mangle]
pub unsafe extern "C" fn connection_settings_get_use_tls(
    this: *const ConnectionSettings,
) -> OptionalBool {
    if this.is_null() {
        return OptionalBool::None;
    }
    let settings: &ConnectionSettings = { &*this };

    OptionalBool::from_option(settings.use_tls)
}
