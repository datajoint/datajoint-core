use crate::results::TableRow;
use futures::stream::StreamExt;
use futures_core::stream::BoxStream;

type SqlxExecutor = sqlx::AnyPool;
type SqlxCursor<'c> = BoxStream<'c, Result<sqlx::any::AnyRow, sqlx::Error>>;

/// An object used to interact with a database by executing queries.
///
/// Instances of `Cursor` should not be created manually but by calling a query method
/// on a `Connection` instance.
pub struct Cursor<'c> {
    executor: &'c SqlxExecutor,
    runtime: &'c tokio::runtime::Runtime,
    stream: Option<SqlxCursor<'c>>,
}

impl<'c> Cursor<'c> {
    /// Creates a new cursor over the given SQLx executor.
    pub(crate) fn new(executor: &'c SqlxExecutor, runtime: &'c tokio::runtime::Runtime) -> Self {
        Cursor {
            executor: executor,
            runtime: runtime,
            stream: None,
        }
    }

    /// Executes the given query over the connection.
    pub fn execute(&mut self, query: &'c str) {
        self.stream = Some(sqlx::query(query).fetch(self.executor));

    }

    /// Fetches the next row from the previous query.
    ///
    /// Panics on error.
    pub fn fetch_one(&mut self) -> TableRow {
        self.try_fetch_one().unwrap()
    }

    /// Fetches the next row from the previous query.
    pub fn try_fetch_one(&mut self) -> Result<TableRow, &str> {
        match &mut self.stream {
            None => Err("error in fetch_one 1"),
            Some(ref mut stream) => match self.runtime.block_on(stream.next()) {
                None => Err("error in fetch_one 2"),
                Some(result) => match result {
                    Err(_) => Err("error in fetch_one 3"),
                    Ok(row) => Ok(TableRow::new(row)),
                },
            },
        }
    }

    /// Fetches all remaining rows from the previous query.
    ///
    /// Panics on error.
    pub fn fetch_all(&mut self) -> Vec<TableRow> {
        self.try_fetch_all().unwrap()
    }

    /// Fetches all remaining rows from the previous query.
    pub fn try_fetch_all(&mut self) -> Result<Vec<TableRow>, &str> {
        let mut rows = vec![];
        while let Ok(row) = self.try_fetch_one() {
            rows.push(row);
        }

        Ok(rows)
    }
}
