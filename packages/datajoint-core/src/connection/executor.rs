use crate::connection::{Cursor, NativeCursor};
use crate::error::{Error, SqlxError};
use crate::placeholders::PlaceholderArgumentVector;
use crate::results::TableRow;
use sqlx::Executor as SqlxExecutor;

/// An object used to interact with a database by executing queries.
///
/// Instances of `Executor` should not be created manually but by calling
/// `executor()` on a `Connection` instance.
pub struct Executor<'c> {
    // TODO(jackson-nestelroad): Somehow wrap sqlx::AnyExecutor so that pools,
    // connections, and transactions can all use this API.
    pub(crate) executor: &'c sqlx::AnyPool,
    pub(crate) runtime: &'c tokio::runtime::Runtime,
}

impl<'c> Executor<'c> {
    /// Creates a new executor over the given SQLx executor.
    pub(crate) fn new(executor: &'c sqlx::AnyPool, runtime: &'c tokio::runtime::Runtime) -> Self {
        Executor {
            executor: executor,
            runtime: runtime,
        }
    }

    /// Executes the given query over the connection.
    ///
    /// Panics on error.
    pub fn execute(&self, query: &str) -> u64 {
        self.try_execute(query).unwrap()
    }

    /// Executes the given query over the connection.
    pub fn try_execute(&self, query: &str) -> Result<u64, Error> {
        match self.runtime.block_on(self.executor.execute(query)) {
            Err(err) => Err(SqlxError::new(err)),
            Ok(result) => Ok(result.rows_affected()),
        }
    }

    /// Executes the given query over the connection with placeholders.
    pub fn execute_ph(&self, query: &str, args: PlaceholderArgumentVector) -> u64 {
        self.try_execute_ph(query, args).unwrap()
    }

    /// Executes the given query over the connection.
    pub fn try_execute_ph(
        &self,
        query: &str,
        args: PlaceholderArgumentVector,
    ) -> Result<u64, Error> {
        let qu = args.prepare(query);
        match self.runtime.block_on(qu.execute(self.executor)) {
            Err(err) => Err(SqlxError::new(err)),
            Ok(result) => Ok(result.rows_affected()),
        }
    }

    /// Fetches one row using the given query.
    ///
    /// Panics on error.
    pub fn fetch_one(&self, query: &str) -> TableRow {
        self.try_fetch_one(query).unwrap()
    }

    /// Fetches one row using the given query.
    pub fn try_fetch_one(&self, query: &str) -> Result<TableRow, Error> {
        match self.runtime.block_on(self.executor.fetch_one(query)) {
            Err(err) => Err(SqlxError::new(err)),
            Ok(row) => Ok(TableRow::new(row)),
        }
    }

    /// Fetches multiple rows using the given query.
    ///
    /// Panics on error.
    pub fn fetch_all(&self, query: &str) -> Vec<TableRow> {
        self.try_fetch_all(query).unwrap()
    }

    /// Fetches multiple rows using the given query.
    pub fn try_fetch_all(&self, query: &str) -> Result<Vec<TableRow>, Error> {
        match self.runtime.block_on(self.executor.fetch_all(query)) {
            Err(err) => Err(SqlxError::new(err)),
            Ok(rows) => Ok(rows.into_iter().map(TableRow::new).collect()),
        }
    }

    // Creates a cursor for the given query.
    pub fn cursor(&'c self, query: &str) -> Cursor<'c> {
        NativeCursor::new_from_executor_ref(query, &self)
    }

    pub fn cursor_ph(&'c self, query: &str, args: PlaceholderArgumentVector) -> Cursor<'c> {
        NativeCursor::new_from_executor_ref_ph(query, &self, args)
    }
}
