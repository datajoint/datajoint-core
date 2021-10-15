use crate::connection::{Cursor};
use crate::error::{Error, SqlxError};
use crate::results::TableRow;
use sqlx::Executor as SqlxExecutor;
use std::borrow::Borrow;
use crate::placeholders::{PhArg, PlaceHolderArgumentVector};

/// An object used to interact with a database by executing queries.
///
/// Instances of `Executor` should not be created manually but by calling
/// `executor()` on a `Connection` instance.
pub struct Executor<'c> {
    // TODO(jackson-nestelroad): Somehow wrap sqlx::AnyExecutor so that pools,
    // connections, and transactions can all use this API.
    executor: &'c sqlx::AnyPool,
    runtime: &'c tokio::runtime::Runtime,
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

    /// executes the given query over the connection
    pub fn ph_execute(&self, query: &str, args : PlaceHolderArgumentVector) -> u64 {
        self.ph_try_execute(query, args).unwrap()
    }

    /// Executes the given query over the connection.
    pub fn ph_try_execute(&self, query: &str, args : PlaceHolderArgumentVector) -> Result<u64, Error> {
        let qu = args.prepare(query);
        match self.runtime.block_on(qu.execute(self.executor)  ) {
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
    pub fn cursor(&self, query: &'c str) -> Cursor<'c> {
        Cursor::new(self.runtime, sqlx::query(query).fetch(self.executor))
    }

    pub fn ph_cursor(&self, query: &'c str, args: PlaceHolderArgumentVector) -> Cursor<'c> {
        let qu = args.prepare(query);
        Cursor::new(self.runtime, qu.fetch(self.executor))
    }
}
