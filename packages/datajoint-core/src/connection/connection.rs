use crate::connection::{Cursor, Executor};
use sqlx::{Execute, AnyPool};
use crate::error::{DataJointError, Error, ErrorCode, SqlxError};
use crate::placeholders::{PhArg, PlaceHolderArgumentVector};


/// A single connection instance to an arbitrary SQL database.
pub struct Connection {
    // TODO(jonathanschroeter): Replace with settings object, and use to build URI.
    settings: String,
    pool: Option<sqlx::AnyPool>,
    runtime: tokio::runtime::Runtime,
}

impl Connection {
    /// Creates a new connection to a SQL database based on the given settings.
    ///
    /// The connection is not actually established until [.connect()][Connection::connect]
    /// is called.
    pub fn new(settings: String) -> Self {
        Connection {
            settings: settings,
            pool: None,
            runtime: tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .ok()
                .unwrap(),
        }
    }

    /// Starts the connection to the SQL database according to settings the object was
    /// initialized with.
    pub fn connect(&mut self) -> Result<(), Error> {
        self.pool = Some(Connection::get_pool(&self.runtime, &*self.settings)?);
        return Ok(());
    }

    /// Disconnects from the SQL database.
    ///
    /// The connection can be restarted if desired.
    ///
    /// The connection is not deallocated
    pub fn disconnect(& self) {
        // TODO(jnestelroad): Implement with self.pool.close() async.
        match &self.pool {
            Some(p) => self.runtime.block_on(p.close()),
            _ => {}
        }
    }

    fn get_pool(
        runtime: &tokio::runtime::Runtime,
        uri: &str,
    ) -> Result<sqlx::AnyPool, Error> {
        runtime.block_on(Connection::get_pool_async(uri))
    }

    async fn get_pool_async(uri: &str) -> Result<sqlx::AnyPool,Error> {
        match sqlx::any::AnyPoolOptions::new()
            // TODO(jnestelroad): Allow more than one connection in settings?
            .max_connections(1)
            .connect(uri)
            .await
        {
            Err(err) => Err(SqlxError::new(err)),
            Ok(pool) => Ok(pool),
        }
    }

    /// Creates an executor to interact with the database over this connection.
    ///
    /// Panics on error.
    pub fn executor(&self) -> Executor {
        self.try_executor().unwrap()
    }

    /// Creates an executor to interact with the database over this connection.
    pub fn try_executor(&self) -> Result<Executor, Error> {

        match &self.pool {
            None =>  Err(DataJointError::new("not connected",
                ErrorCode::NotConnected)),
            Some(pool) => Ok(Executor::new(pool, &self.runtime)),
        }
    }

    /// Executes the given non-returning query, returning the number of rows affected.
    ///
    /// Panics on error.
    pub fn execute_query(&self, query: &str) -> u64 {
        
        self.try_execute_query(query).unwrap()
    }


    /// Executes the given non-returning query, returning the number of rows affected.
    pub fn try_execute_query(&self, query: &str) -> Result<u64, Error> {
            Ok(self.try_executor()?.try_execute(query)?)
    }


    pub fn ph_execute_query(&self, query: &str, args : PlaceHolderArgumentVector) -> u64 {

        self.ph_try_execute_query(query, args).unwrap()
    }

    pub fn ph_try_execute_query(&self, query: &str, args : PlaceHolderArgumentVector) -> Result<u64, Error> {
        Ok(self.try_executor()?.ph_try_execute(query, args)?)
    }

    /// Creates a cursor for iterating over the results of the given returning query.
    ///
    /// Panics on error.
    pub fn fetch_query<'c>(&'c self, query: &'c str) -> Cursor {
        self.try_fetch_query(query).unwrap()
    }

    pub fn ph_fetch_query<'c>(&'c self, query: &'c str , args : PlaceHolderArgumentVector) -> Cursor {
        self.ph_try_fetch_query(query, args).unwrap()
    }


    /// Creates a cursor for iterating over the results of the given returning query.
    pub fn try_fetch_query<'c>(&'c self, query: &'c str) -> Result<Cursor, Error> {
        Ok(self.try_executor()?.cursor(query))
    }

    pub fn ph_try_fetch_query<'c>(&'c self, query: &'c str, args : PlaceHolderArgumentVector) -> Result<Cursor, Error> {
        Ok(self.try_executor()?.ph_cursor(query,args))
    }

}
