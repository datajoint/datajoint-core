use crate::common::{DatabaseType, DatabaseTypeAgnostic};
use crate::connection::Pool;
use crate::connection::{ConnectionSettings, Cursor, Executor};
use crate::error::{DataJointError, Error, ErrorCode, SqlxError};
use crate::placeholders::{PlaceholderArgumentCollection, PlaceholderArgumentVector};

/// A single connection instance to an arbitrary SQL database.
pub struct Connection {
    /// The settings for the database connection.
    ///
    /// If changes to the settings are made after a connection has been established,
    /// the client should call [`.disconnect()`][Connection::disconnect] and then connect
    /// again to use the updated settings.
    pub settings: ConnectionSettings,
    pool: Option<Pool>,
    runtime: tokio::runtime::Runtime,
}

impl DatabaseTypeAgnostic for Connection {
    fn database_type(&self) -> DatabaseType {
        match &self.pool {
            None => self.settings.database_type,
            Some(pool) => pool.database_type(),
        }
    }
}

impl Connection {
    /// Creates a new connection to a SQL database based on the given settings.
    ///
    /// The connection is not actually established until [`.connect()`][Connection::connect]
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

    /// Starts the connection to the SQL database according to the settings the connection
    /// was initialized with.
    pub fn connect(&mut self) -> Result<(), Error> {
        self.pool = Some(Connection::get_pool(
            &self.runtime,
            self.settings.database_type,
            &*self.settings.uri(),
        )?);
        return Ok(());
    }

    fn not_connected_error() -> Error {
        DataJointError::new(ErrorCode::NotConnected)
    }

    fn get_connected_pool(&self) -> Result<&Pool, Error> {
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

    fn get_pool(
        runtime: &tokio::runtime::Runtime,
        database_type: DatabaseType,
        uri: &str,
    ) -> Result<Pool, Error> {
        runtime.block_on(Connection::get_pool_async(database_type, uri))
    }

    async fn get_pool_async(database_type: DatabaseType, uri: &str) -> Result<Pool, Error> {
        match database_type {
            DatabaseType::MySql => {
                match sqlx::mysql::MySqlPoolOptions::new()
                    .max_connections(1)
                    .connect(uri)
                    .await
                {
                    Err(err) => Err(SqlxError::new(err)),
                    Ok(pool) => Ok(Pool::MySql(pool)),
                }
            }
            DatabaseType::Postgres => {
                match sqlx::postgres::PgPoolOptions::new()
                    .max_connections(1)
                    .connect(uri)
                    .await
                {
                    Err(err) => Err(SqlxError::new(err)),
                    Ok(pool) => Ok(Pool::Postgres(pool)),
                }
            }
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
    ///
    /// Uses placeholder arguments, binding them to the query prior to execution.
    ///
    /// Panics on error.
    pub fn execute_query_ph(&self, query: &str, args: impl PlaceholderArgumentCollection) -> u64 {
        self.try_execute_query_ph(query, args).unwrap()
    }

    /// Executes the given non-returning query, returning the number of rows affected.
    pub fn try_execute_query(&self, query: &str) -> Result<u64, Error> {
        Ok(self.try_executor()?.try_execute(query)?)
    }

    /// Executes the given non-returning query, returning the number of rows affected.
    ///
    /// Uses placeholder arguments, binding them to the query prior to execution.
    pub fn try_execute_query_ph(
        &self,
        query: &str,
        args: impl PlaceholderArgumentCollection,
    ) -> Result<u64, Error> {
        Ok(self.try_executor()?.try_execute_ph(query, args)?)
    }

    /// Creates a cursor for iterating over the results of the given returning query.
    ///
    /// Panics on error.
    pub fn fetch_query<'c>(&'c self, query: &str) -> Cursor<'c> {
        self.try_fetch_query(query).unwrap()
    }

    /// Creates a cursor for iterating over the results of the given returning query.
    ///
    /// Uses placeholder arguments, binding them to the query prior to execution.
    ///
    /// Panics on error.
    pub fn fetch_query_ph<'c>(
        &'c self,
        query: &'c str,
        args: impl PlaceholderArgumentCollection,
    ) -> Cursor {
        self.try_fetch_query_ph(query, args).unwrap()
    }

    /// Creates a cursor for iterating over the results of the given returning query.
    pub fn try_fetch_query<'c>(&'c self, query: &str) -> Result<Cursor<'c>, Error> {
        Cursor::new_from_executor(
            query,
            self.try_executor()?,
            None as Option<PlaceholderArgumentVector>,
        )
    }

    /// Creates a cursor for iterating over the results of the given returning query.
    ///
    /// Uses placeholder arguments, binding them to the query prior to execution.
    pub fn try_fetch_query_ph<'c>(
        &'c self,
        query: &'c str,
        args: impl PlaceholderArgumentCollection,
    ) -> Result<Cursor, Error> {
        Cursor::new_from_executor(query, self.try_executor()?, Some(args))
    }
}
