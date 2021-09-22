extern crate libc;

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
            pool:None,
            options: PgPoolOptions::new(),
            run_time : tokio::runtime::Builder::new_current_thread()
                .enable_all().build().ok().unwrap()
        }
    }


    pub fn connect(&mut self){
        self.pool = Some(self.run_time.block_on(
            self.get_pool_async()
        ));
        println!("successfully connected")
    }

    pub fn query(& self , query : &str){
        self.run_time.block_on(self.query_async(query));
    }

    pub async fn query_async(&self, query : & str ) -> Result<(), sqlx::Error> {

        println!("{}",query);

        let row: (i32,) = sqlx::query_as(query)
            .fetch_one(   self.pool.as_ref().unwrap())
            .await.map_err(|e|{
            println!("{:?} failed to fetch", e)
        }).ok().unwrap();

        println!("fetched value {:?}",row.0);
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

        println!("{}",uri);

        PgPoolOptions::new()
            .max_connections(1)
            .connect(&*uri).await.map_err(|e|{
            println!("failed to connect {:?}", e)
        }).ok()
            .unwrap()


    }



    #[no_mangle]
    pub extern "C" fn connection_new() -> *mut Connection {
        Box::into_raw(Box::new(Connection::new()))
    }

    #[no_mangle]
    pub extern "C" fn connection_free(ptr: *mut Connection) {
        if ptr.is_null() {
            return;
        }
        unsafe {
            Box::from_raw(ptr);

        }
    }

    #[no_mangle]
    pub extern "C" fn connection_connect(
        ptr: *mut Connection,
    ) {

        let database = unsafe {
            assert!(!ptr.is_null());
            &mut *ptr
        };
        // let host = unsafe {
        //     assert!(!host.is_null());
        //     CStr::from_ptr(host)
        // };
        // let host_str = host.to_str().unwrap();
        //
        // let user = unsafe {
        //     assert!(!user.is_null());
        //     CStr::from_ptr(user)
        // };
        // let user_str = user.to_str().unwrap();
        //
        // let password = unsafe {
        //     assert!(!password.is_null());
        //     CStr::from_ptr(password)
        // };
        // let password_str = password.to_str().unwrap();


        database.connect();
        // should this be returning something ???
    }

    #[no_mangle]
    pub extern "C" fn connection_query(
        ptr: *mut Connection,
        query: *const c_char
    ) {
        let database = unsafe {
            assert!(!ptr.is_null());
            &*ptr
        };

        let query = unsafe {
            assert!(!query.is_null());
            CStr::from_ptr(query)
        };

        let query_str = query.to_str().unwrap();
        database.query(query_str)
    }

}