use crate::common::{DatabaseType, DatabaseTypeAgnostic};
use crate::error::{DataJointError, Error, ErrorCode, SqlxError};
use crate::query::Query;
use crate::results::TableRow;
use sqlx::Executor;

// TODO(jackson-nestelroad): Somehow wrap `sqlx::Executor` instead of `sqlx::Pool` (which
// implements `sqlx::Executor`) to make this type more generic, allowing more types to use
// the same common interface.

/// A connection pool in SQLx.
///
/// Currently only used to implement an executor-like interface.
///
/// Different from [`Executor`][crate::connection::Executor] in that
/// it does not rely on a runtime for asynchronous operations.
pub(crate) enum Pool {
    MySql(sqlx::Pool<sqlx::MySql>),
    Postgres(sqlx::Pool<sqlx::Postgres>),
}

impl DatabaseTypeAgnostic for Pool {
    fn database_type(&self) -> DatabaseType {
        match self {
            Self::MySql(_) => DatabaseType::MySql,
            Self::Postgres(_) => DatabaseType::Postgres,
        }
    }
}

impl Pool {
    /// Checks if the connection pool has been closed.
    pub fn is_closed(&self) -> bool {
        match self {
            Self::MySql(pool) => pool.is_closed(),
            Self::Postgres(pool) => pool.is_closed(),
        }
    }

    /// Closes the connection.
    pub async fn close(&self) {
        match self {
            Self::MySql(pool) => pool.close().await,
            Self::Postgres(pool) => pool.close().await,
        }
    }

    fn wrong_database_type_error() -> Error {
        DataJointError::new_with_message(
            "prepared query is for the wrong database type",
            ErrorCode::WrongDatabaseType,
        )
    }

    /// Attempts to execute a non-returning query over the connection.
    ///
    /// Returns the number of rows affected by the query.
    pub async fn try_execute<'q>(&self, query: Query<'q>) -> Result<u64, Error> {
        match self {
            Self::MySql(pool) => {
                if let Query::MySql(query) = query {
                    match pool.execute(query).await {
                        Err(error) => Err(SqlxError::new(error)),
                        Ok(result) => Ok(result.rows_affected()),
                    }
                } else {
                    Err(Pool::wrong_database_type_error())
                }
            }
            Self::Postgres(pool) => {
                if let Query::Postgres(query) = query {
                    match pool.execute(query).await {
                        Err(error) => Err(SqlxError::new(error)),
                        Ok(result) => Ok(result.rows_affected()),
                    }
                } else {
                    Err(Pool::wrong_database_type_error())
                }
            }
        }
    }

    /// Attempts to execute a returning query over the connection.
    ///
    /// Returns a single row returned by the query.
    pub async fn try_fetch_one<'q>(&self, query: Query<'q>) -> Result<TableRow, Error> {
        match self {
            Self::MySql(pool) => {
                if let Query::MySql(query) = query {
                    match pool.fetch_one(query).await {
                        Err(error) => Err(SqlxError::new(error)),
                        Ok(row) => Ok(TableRow::MySql(row)),
                    }
                } else {
                    Err(Pool::wrong_database_type_error())
                }
            }
            Self::Postgres(pool) => {
                if let Query::Postgres(query) = query {
                    match pool.fetch_one(query).await {
                        Err(error) => Err(SqlxError::new(error)),
                        Ok(row) => Ok(TableRow::Postgres(row)),
                    }
                } else {
                    Err(DataJointError::new_with_message(
                        "prepared query is for the wrong database type",
                        ErrorCode::WrongDatabaseType,
                    ))
                }
            }
        }
    }

    /// Attempts to execute a returning query over the connection.
    ///
    /// Returns all rows returned by the query.
    pub async fn try_fetch_all<'q>(&self, query: Query<'q>) -> Result<Vec<TableRow>, Error> {
        match self {
            Self::MySql(pool) => {
                if let Query::MySql(query) = query {
                    match pool.fetch_all(query).await {
                        Err(error) => Err(SqlxError::new(error)),
                        Ok(rows) => Ok(rows.into_iter().map(TableRow::MySql).collect()),
                    }
                } else {
                    Err(Pool::wrong_database_type_error())
                }
            }
            Self::Postgres(pool) => {
                if let Query::Postgres(query) = query {
                    match pool.fetch_all(query).await {
                        Err(error) => Err(SqlxError::new(error)),
                        Ok(rows) => Ok(rows.into_iter().map(TableRow::Postgres).collect()),
                    }
                } else {
                    Err(Pool::wrong_database_type_error())
                }
            }
        }
    }
}
