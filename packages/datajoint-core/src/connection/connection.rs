use crate::connection::cursor::Cursor;
use crate::connection::settings::ConnectionSettings;

/// A single connection instance to an arbitrary SQL database.
pub struct Connection {
   //changed to ConnectionSettings object
    settings: ConnectionSettings,
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
            //setting settings to the past in instance
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
        //using settings.uri to get the uri connection string
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

    /// Creates a cursor to interact with the database over this connection.
    ///
    /// Panics on error.
    pub fn cursor<'c>(&'c self) -> Cursor<'c> {
        self.try_cursor().unwrap()
    }

    /// Creates a cursor to interact with the database over this connection.
    pub fn try_cursor<'c>(&'c self) -> Result<Cursor<'c>, &str> {
        match &self.pool {
            None => Err("error in cursor"),
            Some(pool) => Ok(Cursor::new(pool, &self.runtime)),
        }
    }

    /// Creates a cursor for a raw query, given as a string, to execute over the connection.
    ///
    /// Panics on error.
    pub fn raw_query<'c>(&'c self, query: &'c str) -> Cursor<'c> {
        self.try_raw_query(query).unwrap()
    }

    // Creates a cursor for a raw query, given as a string, to execute over the connection.
    pub fn try_raw_query<'c>(&'c self, query: &'c str) -> Result<Cursor<'c>, &str> {
        self.runtime.block_on(self.raw_query_async(query))
    }

    async fn raw_query_async<'c>(&'c self, query: &'c str) -> Result<Cursor<'c>, &str> {
        match &self.pool {
            None => Err("error in query_async"),
            Some(pool) => {
                let mut cursor = Cursor::new(pool, &self.runtime);
                cursor.execute(query);
                Ok(cursor)
            }
        }
    }
}
