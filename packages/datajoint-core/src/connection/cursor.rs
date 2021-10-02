use crate::results::TableRow;
use sqlx::Executor;

type SqlxExecutor = sqlx::AnyPool;

/// An object used to interact with a database by executing queries.
///
/// Instances of `Cursor` should not be created manually but by calling a query method
/// on a `Connection` instance.
pub struct Cursor<'c> {
    executor: &'c SqlxExecutor,
    runtime: &'c tokio::runtime::Runtime,
}

impl<'c> Cursor<'c> {
    /// Creates a new cursor over the given SQLx executor.
    pub(crate) fn new(executor: &'c SqlxExecutor, runtime: &'c tokio::runtime::Runtime) -> Self {
        Cursor {
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
    pub fn try_execute(&self, query: &str) -> Result<u64, &str> {
        match self.runtime.block_on(self.executor.execute(query)) {
            Err(_) => Err("error in try_execute"),
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
    pub fn try_fetch_one(&self, query: &str) -> Result<TableRow, &str> {
        match self.runtime.block_on(self.executor.fetch_one(query)) {
            Err(_) => Err("error in try_fetch_one"),
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
    pub fn try_fetch_all(&self, query: &str) -> Result<Vec<TableRow>, &str> {
        match self.runtime.block_on(self.executor.fetch_all(query)) {
            Err(_) => Err("error in try_fetch_all"),
            Ok(rows) => Ok(rows.into_iter().map(TableRow::new).collect()),
        }
    }
}
