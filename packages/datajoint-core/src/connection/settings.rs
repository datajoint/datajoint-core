/// Enum type for representing the type of SQL database to connect to.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(i32)]
pub enum DatabaseType {
    MySql,
    Postgres,
}

pub struct ConnectionSettings {
    pub database_type: DatabaseType,
    pub username: String,
    pub password: String,
    pub hostname: String,
    pub port: u16,
    pub database_name: String,
    pub use_tls: Option<bool>,
}

impl ConnectionSettings {
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
    pub fn uri(&self) -> String {

        //getting warnings about these variables never being read before being overwritten, not sure how to avoid it here
        let mut protocol = String::new();
        let mut tls_ssl = String::new();

        match self.database_type {
            DatabaseType::Postgres => {
                protocol = "postgres".to_string();
                tls_ssl = "ssl".to_string()
            }
            DatabaseType::MySql => {
                protocol = "mysql".to_string();
                tls_ssl = "tls".to_string()
            }
        }
        //postgres://user:pass@host:port/database?ssl=true
        //mysql://user:pass@host:port/database?tls=true
        match self.use_tls {
            Some(true) => 
                return format!(
                    "{}://{}:{}@{}:{}/{}?{}=true",
                    protocol,
                    self.username,
                    self.password,
                    self.hostname,
                    self.port.to_string(),
                    self.database_name,
                    tls_ssl,
                ),
            Some(false) => 
                return format!(
                    "{}://{}:{}@{}:{}/{}?{}=false",
                    protocol,
                    self.username,
                    self.password,
                    self.hostname,
                    self.port.to_string(),
                    self.database_name,
                    tls_ssl,
                ),
            None => 
                return format!(
                    "{}://{}:{}@{}:{}/{}",
                    protocol,
                    self.username,
                    self.password,
                    self.hostname,
                    self.port.to_string(),
                    self.database_name,
                ),
        } 
    }
}
