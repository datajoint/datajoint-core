use sqlx::postgres::PgPoolOptions;
use sqlx::mysql::MySqlPoolOptions;

pub struct Connection {
}

impl Connection {
    pub fn new() -> Connection {
        Connection {
        }
    }

    // Support for MySQL and Postgres database servers
    // No dynamic database configuration (hard-coded into the code for now perhaps at compile time)
    pub fn connect(&mut self, host: &str, user: &str, password: &str, reset: bool , use_tls: bool) {
        println!("Connected to database: Host: {} User: {} Password: {} Reset: {} use_TLS: {} ", host, user, password, reset, use_tls);
    }

    // Provide a utility in the lib to receive a Generic SQL query and
    //      execute against a relational database server
    // No placeholder arguments (hard-coded into the queries for now)
    pub fn query(&self, query: &str) -> u32 {
        println!("Making query from rust library: {}", query);
        0
    }
}