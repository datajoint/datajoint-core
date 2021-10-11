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
    pub fn try_next(&mut self) -> Result<TableRow, &str> {
        match self.runtime.block_on(self.stream.next()) {
            None => Err("error in try_next 2"),
            Some(result) => match result {
                Err(_) => Err("error in try_next 3"),
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
    pub fn try_rest(&mut self) -> Result<Vec<TableRow>, &str> {
        let mut rows = vec![];
        while let Ok(row) = self.try_next() {
            rows.push(row);
        }

        Ok(rows)
    }
}
