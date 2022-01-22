use crate::common::{DatabaseType, DatabaseTypeAgnostic};
use crate::connection::{Executor, Pool};
use crate::error::{DataJointError, Error, ErrorCode, SqlxError};
use crate::placeholders::PlaceholderArgumentCollection;
use crate::query::Query;
use crate::results::TableRow;
use futures::stream::StreamExt;
use futures_core::stream::BoxStream;
use std::pin::Pin;
use std::ptr::NonNull;

/// A wrapper around a stream of rows from SQLx, which basically represents a cursor.
enum SqlxCursor<'c> {
    MySql(BoxStream<'c, Result<sqlx::mysql::MySqlRow, sqlx::Error>>),
    Postgres(BoxStream<'c, Result<sqlx::postgres::PgRow, sqlx::Error>>),
}

/// An object used to iterate over a set of rows.
pub struct Cursor<'c> {
    // The owned query string.
    query: Pin<Box<String>>,
    // The asynchronous runtime.
    runtime: &'c tokio::runtime::Runtime,
    // The stream of rows, which references the owned query string.
    stream: Option<SqlxCursor<'c>>,
}

impl<'c> DatabaseTypeAgnostic for Cursor<'c> {
    fn database_type(&self) -> DatabaseType {
        match self.stream.as_ref().unwrap() {
            SqlxCursor::MySql(_) => DatabaseType::MySql,
            SqlxCursor::Postgres(_) => DatabaseType::Postgres,
        }
    }
}

impl<'c> Cursor<'c> {
    fn wrong_database_type_error() -> Error {
        DataJointError::new_with_message(
            "prepared query is for the wrong database type",
            ErrorCode::WrongDatabaseType,
        )
    }

    /// Creates a new cursor over a stream of SQLx rows.
    ///
    /// Consumes the input executor.
    pub(crate) fn new_from_executor(
        query: &str,
        executor: Executor<'c>,
        args: Option<impl PlaceholderArgumentCollection>,
    ) -> Result<Cursor<'c>, Error> {
        // self.stream needs to reference self.query in order to work properly.
        // This is because SQLx expects its query string to live as long as the query itself,
        // but we can't make that guarantee when using this wrapper model.
        //
        // Thus, we need to both own the query string and refer to it in the same struct.
        // This is not the best for Rust, so we have to implement a work around here.
        //
        // We pin the query string in memory so it is guaranteed to not move so that the stream's
        // reference to the stored query string is always valid.

        // Create the cursor.
        let mut new_cursor = Cursor {
            query: Box::pin(query.to_string()),
            runtime: executor.runtime,
            stream: None,
        };

        // Create a reference to the owned string.
        let slice = NonNull::from(&*new_cursor.query);

        // We know this is safe because modifying a single field does not move the whole struct.
        unsafe {
            // Create the stream with the reference to the query.
            new_cursor.stream = match executor.executor {
                Pool::MySql(pool) => {
                    let mut query = Query::new(DatabaseType::MySql, slice.as_ref());
                    if let Some(args) = args {
                        query = args.bind_to_query(query)?;
                    }
                    if let Query::MySql(query) = query {
                        Some(SqlxCursor::MySql(query.fetch(pool)))
                    } else {
                        return Err(Cursor::wrong_database_type_error());
                    }
                }
                Pool::Postgres(pool) => {
                    let mut query = Query::new(DatabaseType::Postgres, slice.as_ref());
                    if let Some(args) = args {
                        query = args.bind_to_query(query)?;
                    }
                    if let Query::Postgres(query) = query {
                        Some(SqlxCursor::Postgres(query.fetch(pool)))
                    } else {
                        return Err(Cursor::wrong_database_type_error());
                    }
                }
            }
        }

        return Ok(new_cursor);
    }

    /// Creates a new cursor over a stream of SQLx rows.
    ///
    /// Keeps the executor reference simply by borrowing out of it.
    pub(crate) fn new_from_executor_ref(
        query: &str,
        executor: &'c Executor,
        args: Option<impl PlaceholderArgumentCollection>,
    ) -> Result<Cursor<'c>, Error> {
        // See the above function for what this is doing and why.

        let mut new_cursor = Cursor {
            query: Box::pin(query.to_string()),
            runtime: executor.runtime,
            stream: None,
        };
        let slice = NonNull::from(&*new_cursor.query);

        unsafe {
            new_cursor.stream = match executor.executor {
                Pool::MySql(pool) => {
                    let mut query = Query::new(DatabaseType::MySql, slice.as_ref());
                    if let Some(args) = args {
                        query = args.bind_to_query(query)?;
                    }
                    if let Query::MySql(query) = query {
                        Some(SqlxCursor::MySql(query.fetch(pool)))
                    } else {
                        return Err(Cursor::wrong_database_type_error());
                    }
                }
                Pool::Postgres(pool) => {
                    let mut query = Query::new(DatabaseType::Postgres, slice.as_ref());
                    if let Some(args) = args {
                        query = args.bind_to_query(query)?;
                    }
                    if let Query::Postgres(query) = query {
                        Some(SqlxCursor::Postgres(query.fetch(pool)))
                    } else {
                        return Err(Cursor::wrong_database_type_error());
                    }
                }
            }
        }

        return Ok(new_cursor);
    }

    /// Fetches the next row.
    ///
    /// Panics on error.
    pub fn next(&mut self) -> TableRow {
        self.try_next().unwrap()
    }

    /// Fetches the next row.
    pub fn try_next(&mut self) -> Result<TableRow, Error> {
        match self.stream.as_mut().unwrap() {
            SqlxCursor::MySql(stream) => match self.runtime.block_on(stream.next()) {
                None => Err(DataJointError::new(ErrorCode::NoMoreRows)),
                Some(result) => match result {
                    Err(err) => Err(SqlxError::new(err)),
                    Ok(row) => Ok(TableRow::MySql(row)),
                },
            },
            SqlxCursor::Postgres(stream) => match self.runtime.block_on(stream.next()) {
                None => Err(DataJointError::new(ErrorCode::NoMoreRows)),
                Some(result) => match result {
                    Err(err) => Err(SqlxError::new(err)),
                    Ok(row) => Ok(TableRow::Postgres(row)),
                },
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
