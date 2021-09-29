#[derive(Eq, PartialEq)]
enum DatabaseType {
    MySql,
    Postgres,
}
struct ConnectionSettings {
    database_type: DatabaseType,
    username: String,
    password: String,
    hostname: String,
    port: u16,
    database_name: String,
    use_tls: Option<bool>,
}

impl ConnectionSettings{
    pub fn new() -> ConnectionSettings{
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
