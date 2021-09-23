
use sqlx::postgres::{PgPoolOptions, PgPool, PgRow};
use sqlx::mysql::MySqlPoolOptions;
use tokio::runtime::Runtime;
use std::rc::Rc;
use sqlx::{Pool, Postgres, FromRow, Row};



pub struct Connection {
    host: String,
    user: String,
    password: String,
    reset: bool,
    use_tls: bool,
    // TODO switch to the generic version of Pool and Pool Options
    pool :  Option<Pool<Postgres>>,
    options : PgPoolOptions,
    run_time : Runtime
}


// TODO use jonny's settings module instead of a list of flat settings

impl Connection {
    pub fn new(host: &str, user: &str, password: &str, reset: bool, use_tls: bool) -> Self {
        Connection {
            host: host.to_string(),
            user: user.to_string(),
            password: password.to_string(),
            reset,
            use_tls,
            pool:None,
            options: PgPoolOptions::new(),
            run_time : tokio::runtime::Builder::new_current_thread()
                .enable_all().build().ok().unwrap()
        }
    }

    // Support for MySQL and Postgres database servers
    // No dynamic database configuration (hard-coded into the code for now perhaps at compile time)
    pub fn connect(&mut self) {
        self.pool = Some(self.run_time.block_on(
            self.get_pool_async()
        ));
        println!("Connected to database with the following settings:\n\
                Host: {}\n\
                User: {}\n\
                Password: {}\n\
                Reset: {}\n\
                use_TLS: {} ",
                self.host, self.user, self.password, self.reset, self.use_tls);
    }

    // TODO pass in settings when making this connection
    async fn get_pool_async(&self) -> Pool<Postgres> {

        // let driver = {"PostgreSQL"};
        // let server = "127.0.0.1";
        // let port = 5432;
        // let database = "datajoint";
        // let user = "admin";
        // let pwd = "iameddie";

        //
        // let uri = format!("Driver={};Server={};Port={};Database={};Uid={};Pwd={};"
        //                   ,driver,server,port,database,user,pwd);
        //

        let uri = "postgres://postgres:password@localhost/datajoint";

        println!("{}",uri);

        PgPoolOptions::new()
            .max_connections(1)
            .connect(&*uri).await.map_err(|e|{
            println!("failed to connect {:?}", e)
        }).ok()
            .unwrap()


    }

    // Provide a utility in the lib to receive a Generic SQL query and
    //      execute against a relational database server
    // No placeholder arguments (hard-coded into the queries for now)

    // TODO have raw query return an executor / database cursor
    pub fn raw_query(&self, query: &str) -> i32 {
        println!("Making query from rust library: {}", query);
        self.run_time.block_on(self.query_async(query))

    }

    // TODO return an executor / database cursor
    async fn query_async(&self, query : & str ) -> i32 {

        println!("{}",query);

        let row: (i32,) = sqlx::query_as(query)
            .fetch_one(   self.pool.as_ref().unwrap())
            .await.map_err(|e|{
            println!("{:?} failed to fetch", e)
        }).ok().unwrap();

        row.0
    }
}