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
    fn new() -> ConnectionSettings{
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
    fn uri(&self) -> String {
        //Hardcode in the username, password, and databasename, ect whatever is needed since those are not defaults
        let mut uri = "".to_string();
        //set the default protcall to mysql
        let protocall = "mysql".to_string();
        if self.database_type == DatabaseType::Postgres {
            let protocall = "postgres";
        }
        uri = format!("{}://{}:{}@{}:{}/{}",protocall,self.username,self.password,self.hostname,self.port.to_string(),self.database_name);
        return uri;
    }
}
