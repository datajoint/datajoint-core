use crate::error::{DataJointError, Error, ErrorCode, SqlxError};
use crate::results::TableRow;
use futures::stream::StreamExt;
use futures_core::stream::BoxStream;

type SqlxCursor<'c> = BoxStream<'c, Result<sqlx::any::AnyRow, sqlx::Error>>;

/// An object used to iterate over a set of rows.
pub struct Cursor<'c> {
    runtime: &'c tokio::runtime::Runtime,
    stream: SqlxCursor<'c>,
}

impl<'c> Cursor<'c> {
    /// Creates a new cursor over a stream of SQLx rows.
    pub(crate) fn new(runtime: &'c tokio::runtime::Runtime, stream: SqlxCursor<'c>) -> Self {
        Cursor {
            runtime: runtime,
            stream: stream,
        }
    }

    /// Fetches the next row.
    ///
    /// Panics on error.
    pub fn next(&mut self) -> TableRow {
        self.try_next().unwrap()
    }

    /// Fetches the next row.
    pub fn try_next(&mut self) -> Result<TableRow, Error> {
        match self.runtime.block_on(self.stream.next()) {
            None => Err(DataJointError::new("no more rows", ErrorCode::NoMoreRows)),
            Some(result) => match result {
                Err(err) => Err(SqlxError::new(err)),
                Ok(row) => Ok(TableRow::new(row)),
            },
        }
    }

    /// Fetches all remaining rows.
    ///
    /// Panics on error.
    pub fn rest(&mut self) -> Vec<TableRow> {
        self.try_rest().unwrap()
    }

    /// Fetches all remaining rows.
    pub fn try_rest(&mut self) -> Result<Vec<TableRow>, Error> {
        let mut rows = vec![];
        loop {
            match self.try_next() {
                Ok(row) => rows.push(row),
                Err(err) if err.code() == ErrorCode::NoMoreRows => break,
                Err(err) => return Err(err),
            }
        }

        Ok(rows)
    }
}
