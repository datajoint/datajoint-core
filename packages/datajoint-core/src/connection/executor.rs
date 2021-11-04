use crate::common::{DatabaseType, DatabaseTypeAgnostic};
use crate::connection::{Cursor, NativeCursor, Pool};
use crate::error::Error;
use crate::placeholders::{PlaceholderArgumentCollection, PlaceholderArgumentVector};
use crate::query::Query;
use crate::results::TableRow;

/// An object used to interact with a database by executing queries.
///
/// Instances of `Executor` should not be created manually but by calling
/// `executor()` on a `Connection` instance.
pub struct Executor<'c> {
    // TODO(jackson-nestelroad): Somehow wrap sqlx::Executor so that pools,
    // connections, and transactions can all use this API.
    pub(crate) executor: &'c Pool,
    pub(crate) runtime: &'c tokio::runtime::Runtime,
}

impl<'c> DatabaseTypeAgnostic for Executor<'c> {
    fn database_type(&self) -> DatabaseType {
        self.executor.database_type()
    }
}

impl<'c> Executor<'c> {
    /// Creates a new executor over the given SQLx executor.
    pub(crate) fn new(executor: &'c Pool, runtime: &'c tokio::runtime::Runtime) -> Self {
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
    ///
    /// Uses placeholder arguments, binding them to the query prior to execution.
    pub fn execute_ph(&self, query: &str, args: impl PlaceholderArgumentCollection) -> u64 {
        self.try_execute_ph(query, args).unwrap()
    }

    /// Executes the given query over the connection.
    pub fn try_execute(&self, query: &str) -> Result<u64, Error> {
        self.runtime.block_on(
            self.executor
                .try_execute(Query::new(self.database_type(), query)),
        )
    }

    /// Executes the given query over the connection.
    ///
    /// Uses placeholder arguments, binding them to the query prior to execution.
    pub fn try_execute_ph(
        &self,
        query: &str,
        args: impl PlaceholderArgumentCollection,
    ) -> Result<u64, Error> {
        self.runtime.block_on(
            self.executor
                .try_execute(args.bind_to_query(Query::new(self.database_type(), query))?),
        )
    }

    /// Fetches one row using the given query.
    ///
    /// Panics on error.
    pub fn fetch_one(&self, query: &str) -> TableRow {
        self.try_fetch_one(query).unwrap()
    }

    /// Fetches one row using the given query.
    pub fn try_fetch_one(&self, query: &str) -> Result<TableRow, Error> {
        self.runtime.block_on(
            self.executor
                .try_fetch_one(Query::new(self.database_type(), query)),
        )
    }

    /// Fetches multiple rows using the given query.
    ///
    /// Panics on error.
    pub fn fetch_all(&self, query: &str) -> Vec<TableRow> {
        self.try_fetch_all(query).unwrap()
    }

    /// Fetches multiple rows using the given query.
    pub fn try_fetch_all(&self, query: &str) -> Result<Vec<TableRow>, Error> {
        self.runtime.block_on(
            self.executor
                .try_fetch_all(Query::new(self.database_type(), query)),
        )
    }

    /// Creates a cursor for the given query.
    pub fn cursor(&'c self, query: &str) -> Result<Cursor<'c>, Error> {
        NativeCursor::new_from_executor_ref(query, &self, None as Option<PlaceholderArgumentVector>)
    }

    /// Creates a cursor for the given query.
    ///
    /// Uses placeholder arguments, binding them to the query prior to execution.
    pub fn cursor_ph(
        &'c self,
        query: &str,
        args: impl PlaceholderArgumentCollection,
    ) -> Result<Cursor<'c>, Error> {
        NativeCursor::new_from_executor_ref(query, self, Some(args))
    }
}
