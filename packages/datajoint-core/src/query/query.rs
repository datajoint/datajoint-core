use crate::common::{DatabaseType, DatabaseTypeAgnostic};

/// Type trait for indicating if a type is safe to be encoded into a query.
///
/// Currently only implements how SQLx type checks input types.
pub trait ValueEncodable<'r>:
    sqlx::Encode<'r, sqlx::MySql>
    + sqlx::Type<sqlx::MySql>
    + sqlx::Encode<'r, sqlx::Postgres>
    + sqlx::Type<sqlx::Postgres>
{
}

impl<'r, T> ValueEncodable<'r> for T where
    T: sqlx::Encode<'r, sqlx::MySql>
        + sqlx::Type<sqlx::MySql>
        + sqlx::Encode<'r, sqlx::Postgres>
        + sqlx::Type<sqlx::Postgres>
{
}

/// A wrapper around a SQLx query.
///
/// You probably should not use this class on its own, as it does not provide
/// any high-level API at the moment. The [`Executor`][crate::connection::Executor]
/// object takes in string queries and placeholder arguments individually,
/// automatically creating this object for execution behind the scenes.
pub enum Query<'q> {
    MySql(
        sqlx::query::Query<
            'q,
            sqlx::MySql,
            <sqlx::MySql as sqlx::database::HasArguments<'q>>::Arguments,
        >,
    ),
    Postgres(
        sqlx::query::Query<
            'q,
            sqlx::Postgres,
            <sqlx::Postgres as sqlx::database::HasArguments<'q>>::Arguments,
        >,
    ),
}

impl<'q> DatabaseTypeAgnostic for Query<'q> {
    fn database_type(&self) -> DatabaseType {
        match self {
            Self::MySql(_) => DatabaseType::MySql,
            Self::Postgres(_) => DatabaseType::Postgres,
        }
    }
}

impl<'q> Query<'q> {
    /// Creates a new SQLx query with the given string.
    ///
    /// The database type is required to assure the query is checked against the correct
    /// database.
    pub fn new(database_type: DatabaseType, query: &'q str) -> Self {
        match database_type {
            DatabaseType::MySql => Self::MySql(sqlx::query::<sqlx::MySql>(query)),
            DatabaseType::Postgres => Self::Postgres(sqlx::query::<sqlx::Postgres>(query)),
        }
    }
}
