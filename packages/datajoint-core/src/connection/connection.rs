use crate::connection::{ConnectionSettings, Cursor, Executor, NativeCursor};
use crate::error::{DataJointError, Error, ErrorCode, SqlxError};

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

    fn not_connected_error() -> Error {
        DataJointError::new("not connected", ErrorCode::NotConnected)
    }

    fn get_connected_pool(&self) -> Result<&sqlx::AnyPool, Error> {
        match &self.pool {
            None => Err(Connection::not_connected_error()),
            Some(pool) => {
                if pool.is_closed() {
                    Err(Connection::not_connected_error())
                } else {
                    Ok(pool)
                }
            }
        }
    }

    /// Checks if the connection is still connected.
    pub fn is_connected(&self) -> bool {
        match self.get_connected_pool() {
            Err(_) => false,
            Ok(_) => true,
        }
    }

    /// Starts the connection to the SQL database according to settings the object was
    /// initialized with.
    pub fn connect(&mut self) -> Result<(), Error> {
        self.pool = Some(Connection::get_pool(&self.runtime, &*self.settings.uri())?);
        return Ok(());
    }

    /// Disconnects from the SQL database.
    ///
    /// If the database connection has already been disconnected, this method
    /// is a no-op.
    ///
    /// The connection can be restarted if desired.
    pub fn disconnect(&self) {
        if let Some(pool) = &self.pool {
            if !pool.is_closed() {
                self.runtime.block_on(pool.close());
            }
        }
    }

    fn get_pool(runtime: &tokio::runtime::Runtime, uri: &str) -> Result<sqlx::AnyPool, Error> {
        runtime.block_on(Connection::get_pool_async(uri))
    }

    async fn get_pool_async(uri: &str) -> Result<sqlx::AnyPool, Error> {
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
    pub fn executor<'c>(&'c self) -> Executor<'c> {
        self.try_executor().unwrap()
    }

    /// Creates an executor to interact with the database over this connection.
    pub fn try_executor<'c>(&'c self) -> Result<Executor<'c>, Error> {
        Ok(Executor::new(self.get_connected_pool()?, &self.runtime))
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

    /// Creates a cursor for iterating over the results of the given returning query.
    ///
    /// Panics on error.
    pub fn fetch_query<'c>(&'c self, query: &str) -> Cursor<'c> {
        self.try_fetch_query(query).unwrap()
    }

    /// Creates a cursor for iterating over the results of the given returning query.
    pub fn try_fetch_query<'c>(&'c self, query: &str) -> Result<Cursor<'c>, Error> {
        Ok(NativeCursor::new_from_executor(query, self.try_executor()?))
    }
}
