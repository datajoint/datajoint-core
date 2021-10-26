use crate::connection::Executor;
use crate::error::{DataJointError, Error, ErrorCode, SqlxError};
use crate::placeholders::PlaceholderArgumentCollection;
use crate::results::TableRow;
use futures::stream::StreamExt;
use futures_core::stream::BoxStream;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;

type SqlxCursor<'c> = BoxStream<'c, Result<sqlx::any::AnyRow, sqlx::Error>>;

/// A cursor pinned to a single place in memory for safety.
pub type Cursor<'c> = Pin<Box<NativeCursor<'c>>>;

/// An object used to iterate over a set of rows.
pub struct NativeCursor<'c> {
    // The owned query string.
    query: String,
    // The asynchronous runtime.
    runtime: &'c tokio::runtime::Runtime,
    // The stream of rows, which references the owned query string.
    stream: Option<SqlxCursor<'c>>,
    // Indicates to not auto-implement Unpin, which assures the memory stays pinned.
    _pin: PhantomPinned,
}

impl<'c> NativeCursor<'c> {
    /// Creates a new cursor over a stream of SQLx rows.
    ///
    /// Consumes the input executor.
    pub(crate) fn new_from_executor(query: &str, executor: Executor<'c>) -> Cursor<'c> {
        // self.stream needs to reference self.query in order to work properly.
        // This is because SQLx expects its query string to live as long as the query itself,
        // but we can't make that guarantee when using this wrapper model.
        //
        // Thus, we need to both own the query string and refer to it in the same struct.
        // This is not the best for Rust, so we have to implement a work around here.
        //
        // We pin the cursor in memory so it is guaranteed to not move so that the stream's
        // reference to the stored query string is always valid. We do not implement Unpin
        // to keep this assumption valid for the cursor's lifetime.alloc

        // Create the cursor.
        let res = NativeCursor {
            query: query.to_string(),
            runtime: executor.runtime,
            stream: None,
            _pin: PhantomPinned,
        };
        // Pin the cursor to a single point in memory.
        let mut boxed = Box::pin(res);

        // Create a reference to the owned string.
        let slice = NonNull::from(&boxed.query);

        // We know this is safe because modifying a single field does not move the whole struct.
        unsafe {
            // Get mutable reference to the created object.
            let unchecked_mut = Pin::get_unchecked_mut(Pin::as_mut(&mut boxed));
            // Create the stream with the reference to the query.
            unchecked_mut.stream = Some(sqlx::query(slice.as_ref()).fetch(executor.executor));
        }

        // Output is the pinned cursor.
        return boxed;
    }

    /// Creates a new cursor over a stream of SQLx rows.
    ///
    /// Consumes the input executor.
    ///
    /// Uses placeholder arguments, binding them to the query prior to execution.
    pub(crate) fn new_from_executor_ph(
        query: &str,
        executor: Executor<'c>,
        args: impl PlaceholderArgumentCollection,
    ) -> Cursor<'c> {
        // See the above function for an explanation of what this is doing and why.

        let res = NativeCursor {
            query: query.to_string(),
            runtime: executor.runtime,
            stream: None,
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(res);
        let slice = NonNull::from(&boxed.query);
        unsafe {
            let query = args.prepare(slice.as_ref());
            let unchecked_mut = Pin::get_unchecked_mut(Pin::as_mut(&mut boxed));
            unchecked_mut.stream = Some(query.fetch(executor.executor));
        }
        return boxed;
    }

    /// Creates a new cursor over a stream of SQLx rows.
    ///
    /// Keeps the executor reference simply by borrowing out of it.
    pub(crate) fn new_from_executor_ref(query: &str, executor: &'c Executor) -> Cursor<'c> {
        // See the above functions for an explanation of what this is doing and why.

        let res = NativeCursor {
            query: query.to_string(),
            runtime: executor.runtime,
            stream: None,
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(res);
        let slice = NonNull::from(&boxed.query);
        unsafe {
            let mut_ref = Pin::as_mut(&mut boxed);
            let unchecked_mut = Pin::get_unchecked_mut(mut_ref);
            unchecked_mut.stream = Some(sqlx::query(slice.as_ref()).fetch(executor.executor));
        }

        return boxed;
    }

    /// Creates a new cursor over a stream of SQLx rows.
    ///
    /// Keeps the executor reference simply by borrowing out of it.
    ///
    /// Uses placeholder arguments, binding them to the query prior to execution.
    pub(crate) fn new_from_executor_ref_ph(
        query: &str,
        executor: &'c Executor,
        args: impl PlaceholderArgumentCollection,
    ) -> Cursor<'c> {
        // See the above functions for an explanation of what this is doing and why.

        let res = NativeCursor {
            query: query.to_string(),
            runtime: executor.runtime,
            stream: None,
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(res);
        let slice = NonNull::from(&boxed.query);
        unsafe {
            let query = args.prepare(slice.as_ref());
            let mut_ref = Pin::as_mut(&mut boxed);
            let unchecked_mut = Pin::get_unchecked_mut(mut_ref);
            unchecked_mut.stream = Some(query.fetch(executor.executor))
        }

        return boxed;
    }

    /// Fetches the next row.
    ///
    /// Panics on error.
    pub fn next(&mut self) -> TableRow {
        self.try_next().unwrap()
    }

    /// Fetches the next row.
    pub fn try_next(&mut self) -> Result<TableRow, Error> {
        match self.runtime.block_on(self.stream.as_mut().unwrap().next()) {
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
