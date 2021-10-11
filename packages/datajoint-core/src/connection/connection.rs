use crate::connection::{Cursor, Executor};
use crate::connection::settings::ConnectionSettings;

/// A single connection instance to an arbitrary SQL database.
pub struct Connection {
    pub settings: ConnectionSettings,
    pool: Option<sqlx::AnyPool>,
    runtime: tokio::runtime::Runtime,
}

impl Connection {
    /// Creates a new connection to a SQL database based on the given settings.
    ///
    /// The connection is not actually established until [.connect()][Connection::connect]
    /// is called.
    pub fn new(settings: ConnectionSettings) -> Self {
        Connection {
            settings,
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
    pub fn connect(&mut self) -> Result<(), &str> {
        self.pool = Some(Connection::get_pool(&self.runtime, &*self.settings.uri())?);
        return Ok(());
    }

    /// Disconnects from the SQL database.
    ///
    /// The connection can be restarted if desired.
    pub fn disconnect(&mut self) -> Result<(), &str> {
        // TODO(jnestelroad): Implement with self.pool.close() async.
        self.pool = None;
        return Ok(());
    }

    fn get_pool(
        runtime: &tokio::runtime::Runtime,
        uri: &str,
    ) -> Result<sqlx::AnyPool, &'static str> {
        runtime.block_on(Connection::get_pool_async(uri))
    }

    async fn get_pool_async(uri: &str) -> Result<sqlx::AnyPool, &'static str> {
        match sqlx::any::AnyPoolOptions::new()
            // TODO(jnestelroad): Allow more than one connection in settings?
            .max_connections(1)
            .connect(uri)
            .await
        {
            Err(_) => Err("failed to get_pool_async"),
            Ok(pool) => Ok(pool),
        }
    }

    /// Creates an executor to interact with the database over this connection.
    ///
    /// Panics on error.
    pub fn executor<'c>(&'c self) -> Executor<'c> {
        self.try_executor().unwrap()
    }

    /// Creates an executor to interact with the database over this connection.
    pub fn try_executor<'c>(&'c self) -> Result<Executor<'c>, &str> {
        match &self.pool {
            None => Err("error in cursor"),
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
    pub fn try_execute_query(&self, query: &str) -> Result<u64, &str> {
        match self.try_executor() {
            Err(_) => Err("error in try_execute_query 1"),
            Ok(executor) => match executor.try_execute(query) {
                Err(_) => Err("error in try_execute_query 2"),
                Ok(rows_affected) => Ok(rows_affected),
            },
        }
    }

    /// Creates a cursor for iterating over the results of the given returning query.
    ///
    /// Panics on error.
    pub fn fetch_query<'c>(&'c self, query: &'c str) -> Cursor {
        self.try_fetch_query(query).unwrap()
    }

    /// Creates a cursor for iterating over the results of the given returning query.
    pub fn try_fetch_query<'c>(&'c self, query: &'c str) -> Result<Cursor, &str> {
        match self.try_executor() {
            Err(_) => Err("error in try_fetch_query 1"),
            Ok(executor) => Ok(executor.cursor(query)),
        }
    }
}
