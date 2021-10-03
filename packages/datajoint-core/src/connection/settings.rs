#[derive(Eq, PartialEq)]
enum DatabaseType {
    MySql,
    Postgres,
}

pub struct ConnectionSettings {
    database_type: DatabaseType,
    username: String,
    password: String,
    hostname: String,
    port: u16,
    database_name: String,
    use_tls: Option<bool>,
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
    

    //The block underneath is for setting the values of a ConnectionSettings instance passed in


    //changing the database_type
    pub fn setsettings_database_type(mut connect: ConnectionSettings, settingsvalue: &str) {
        if settingsvalue == "mysql" || settingsvalue == "Mysql" {
            connect.database_type = DatabaseType::MySql;
        } else {
            connect.database_type = DatabaseType::Postgres;
        }
    }

    //changing the username
    pub fn setsettings_username(mut connect: ConnectionSettings, settingsvalue: &str) {
        connect.username = settingsvalue.to_string();
    }

    //changing the password
    pub fn setsettings_password(mut connect: ConnectionSettings, settingsvalue: &str){
        connect.password = settingsvalue.to_string();
    }

    //changing the hostname
    pub fn setsettings_hostname(mut connect: ConnectionSettings, settingsvalue: &str){
        connect.hostname = settingsvalue.to_string();
    }

    //changing the port
    pub fn setsettings_port(mut connect: ConnectionSettings, settingsvalue: u16){
        connect.port = settingsvalue;
    }

    //changing the database_name
    pub fn setsettings_database_name(mut connect: ConnectionSettings, settingsvalue: &str){
        connect.database_name = settingsvalue.to_string();
    }

    //changing it is uses use_tls
    pub fn setsettings_use_tls(mut connect: ConnectionSettings, settingsvalue: bool){
        if settingsvalue == true {
            connect.use_tls = Some(true);
        } else {
            connect.use_tls = Some(false);
        }
    }
}
