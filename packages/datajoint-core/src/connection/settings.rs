/// Enum type for representing the type of SQL database to connect to.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(i32)]
pub enum DatabaseType {
    MySql,
    Postgres,
}

/// Settings for connecting to an arbitrary SQL database.
pub struct ConnectionSettings {
    /// Type of database to connect to.
    pub database_type: DatabaseType,
    /// Username to login as.
    pub username: String,
    /// Password to use for login.
    pub password: String,
    /// Hostname to connect to.
    pub hostname: String,
    /// Port to connect to.
    pub port: u16,
    /// Name of the database to connec to.
    pub database_name: String,
    /// Whether or not the connection should use TLS to secure the connection.
    pub use_tls: Option<bool>,
}

impl ConnectionSettings {
    /// Creates a new settings instance, initializing the fields with default values.
    pub fn new() -> Self {
        ConnectionSettings {
            database_type: DatabaseType::MySql,
            username: "".to_string(),
            password: "".to_string(),
            hostname: "localhost".to_string(),
            port: 3306,
            database_name: "".to_string(),
            use_tls: None,
        }
    }

    /// Constructs a database connection URI for the settings object.
    pub fn uri(&self) -> String {
        let tls_ssl: &str;
        let mut uri: String;

        match self.database_type {
            DatabaseType::Postgres => {
                uri = "postgres://".to_string();
                tls_ssl = "ssl";
            }
            DatabaseType::MySql => {
                uri = "mysql://".to_string();
                tls_ssl = "tls";
            }
        }
        if !self.username.trim().is_empty() {
            uri.push_str(self.username.as_str());
            if !self.password.trim().is_empty() {
                uri = format!("{}:{}", uri, self.password);
            }
            uri.push('@');
        }
        // Based on the defaults, hostname and port will always have values
        uri = format!("{}{}:{}", uri, self.hostname, self.port);
        if !self.database_name.trim().is_empty() {
            uri = format!("{}/{}", uri, self.database_name);
        }

        match self.use_tls {
            Some(true) => format!("{}?{}=true", uri, tls_ssl),
            Some(false) => format!("{}?{}=false", uri, tls_ssl),
            None => uri,
        }
    }
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_settings_uri() {
        let mut settings = ConnectionSettings::new();
        
        // MySql Test
        assert_eq!(settings.uri(), "mysql://localhost:3306");
        settings.database_type = DatabaseType::MySql;
        assert_eq!(settings.uri(), "mysql://localhost:3306");
        settings.username = "test".to_string();
        assert_eq!(settings.uri(), "mysql://test@localhost:3306");
        settings.password = "testpassword".to_string();
        assert_eq!(settings.uri(), "mysql://test:testpassword@localhost:3306");
        settings.hostname = "testhostname".to_string();
        assert_eq!(settings.uri(), "mysql://test:testpassword@testhostname:3306");
        settings.port = 8800;
        assert_eq!(settings.uri(), "mysql://test:testpassword@testhostname:8800");
        settings.database_name = "testdatabasename".to_string();
        assert_eq!(settings.uri(), "mysql://test:testpassword@testhostname:8800/testdatabasename");
        settings.use_tls = Some(true);
        assert_eq!(settings.uri(), "mysql://test:testpassword@testhostname:8800/testdatabasename?tls=true");
        settings.use_tls = Some(false);
        assert_eq!(settings.uri(), "mysql://test:testpassword@testhostname:8800/testdatabasename?tls=false");
        settings.use_tls = None;
        assert_eq!(settings.uri(), "mysql://test:testpassword@testhostname:8800/testdatabasename");

        // Postgres Test
        let mut settingspost = ConnectionSettings::new();
        settingspost.database_type = DatabaseType::Postgres;
        assert_eq!(settingspost.uri(), "postgres://localhost:3306");
        settingspost.username = "test".to_string();
        assert_eq!(settingspost.uri(), "postgres://test@localhost:3306");
        settingspost.password = "testpassword".to_string();
        assert_eq!(settingspost.uri(), "postgres://test:testpassword@localhost:3306");
        settingspost.hostname = "testhostname".to_string();
        assert_eq!(settingspost.uri(), "postgres://test:testpassword@testhostname:3306");
        settingspost.port = 8800;
        assert_eq!(settingspost.uri(), "postgres://test:testpassword@testhostname:8800");
        settingspost.database_name = "testdatabasename".to_string();
        assert_eq!(settingspost.uri(), "postgres://test:testpassword@testhostname:8800/testdatabasename");
        settingspost.use_tls = Some(true);
        assert_eq!(settingspost.uri(), "postgres://test:testpassword@testhostname:8800/testdatabasename?ssl=true");
        settingspost.use_tls = Some(false);
        assert_eq!(settingspost.uri(), "postgres://test:testpassword@testhostname:8800/testdatabasename?ssl=false");
        settingspost.use_tls = None;
        assert_eq!(settingspost.uri(), "postgres://test:testpassword@testhostname:8800/testdatabasename");
    }
}
