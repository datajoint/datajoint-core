#[derive(Eq, PartialEq)]
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

impl ConnectionSettings{
    pub fn new() -> Self {
        ConnectionSettings{
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
        // Hardcode in the username, password, and databasename, ect whatever is needed since those are not defaults.
        let mut protocol = "mysql".to_string();
        if self.database_type == DatabaseType::Postgres {
            protocol = "postgres".to_string();
        }
        return format!("{}://{}:{}@{}:{}/{}",protocol,self.username,self.password,self.hostname,self.port.to_string(),self.database_name);
    }
}
