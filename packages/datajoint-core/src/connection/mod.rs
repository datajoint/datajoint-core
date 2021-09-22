
use sqlx::postgres::{PgPoolOptions, PgPool, PgRow};
use sqlx::mysql::MySqlPoolOptions;
use std::ptr::{null, null_mut};
use tokio::runtime::Runtime;
use std::rc::Rc;
use sqlx::{Pool, Postgres, FromRow, Row};
use std::os::raw::c_char;
use std::ffi::CStr;


pub struct Connection {
    pool :  Option<Pool<Postgres>>,
    options : PgPoolOptions,
    run_time : Runtime
}

impl Connection {
    pub fn new() -> Connection {
        Connection {
            pool: None,
            options: PgPoolOptions::new(),
            run_time: tokio::runtime::Builder::new_current_thread()
                .enable_all().build().ok().unwrap()
        }
    }


    pub fn connect(&mut self) {
        self.pool = Some(self.run_time.block_on(
            self.get_pool_async()
        ));
        println!("successfully connected")
    }

    pub fn query(&self, query: &str) {
        self.run_time.block_on(self.query_async(query));
    }

    pub async fn query_async(&self, query: &str) -> Result<(), sqlx::Error> {
        println!("{}", query);

        let row: (i32, ) = sqlx::query_as(query)
            .fetch_one(self.pool.as_ref().unwrap())
            .await.map_err(|e| {
            println!("{:?} failed to fetch", e)
        }).ok().unwrap();

        println!("fetched value {:?}", row.0);
        Ok(())
    }


    pub async fn get_pool_async(&self) -> Pool<Postgres> {

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

        println!("{}", uri);

        PgPoolOptions::new()
            .max_connections(1)
            .connect(&*uri).await.map_err(|e| {
            println!("failed to connect {:?}", e)
        }).ok()
            .unwrap()
    }
}