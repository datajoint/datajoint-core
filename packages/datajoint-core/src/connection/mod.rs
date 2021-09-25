// use sqlx::postgres::PgPoolOptions;
// use sqlx::mysql::MySqlPoolOptions;

pub struct Connection {
    host: String,
    user: String,
    password: String,
    reset: bool,
    use_tls: bool
}

impl Connection {
    pub fn new(host: &str, user: &str, password: &str, reset: bool, use_tls: bool) -> Self {
        Connection {
            host: host.to_string(),
            user: user.to_string(),
            password: password.to_string(),
            reset,
            use_tls,
        }
    }

    // Support for MySQL and Postgres database servers
    // No dynamic database configuration (hard-coded into the code for now perhaps at compile time)
    pub fn connect(&mut self) {
        println!("Connected to database with the following settings:\n\
                Host: {}\n\
                User: {}\n\
                Password: {}\n\
                Reset: {}\n\
                use_TLS: {} ",
                self.host, self.user, self.password, self.reset, self.use_tls);
    }

    // Provide a utility in the lib to receive a Generic SQL query and
    //      execute against a relational database server
    // No placeholder arguments (hard-coded into the queries for now)
    pub fn raw_query(&self, query: &str) -> u32 {
        println!("Making query from rust library: {}", query);
        0
    }
}