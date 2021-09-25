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
}

fn createuri() -> String{
    let urimaker = ConnectionSettings::new();
    let mut uri = "".to_string();
    if urimaker.database_type == DatabaseType::MySql {
        uri = format!("mysql://{}:{}@{}:{}/{}",urimaker.username,urimaker.password,urimaker.hostname,urimaker.port.to_string(),urimaker.database_name);
    }else if urimaker.database_type == DatabaseType::Postgres {
        uri = format!("postgres://{}:{}@{}:{}/{}",urimaker.username,urimaker.password,urimaker.hostname,urimaker.port.to_string(),urimaker.database_name);
    } 
    return uri;
}
