use crate::common::{DatabaseType, DatabaseTypeAgnostic};
use crate::error::{Error, SqlxError};
use crate::results::table_column::{ColumnIndex, TableColumnRef};
use sqlx::Row;

/// Type trait for indicating if a type is safe to be decoded to.
///
/// Currently only implements how SQLx type checks return types.
pub trait ValueDecodable<'r>:
    sqlx::Decode<'r, sqlx::MySql> + sqlx::Decode<'r, sqlx::Postgres>
{
}

impl<'r, T> ValueDecodable<'r> for T where
    T: sqlx::Decode<'r, sqlx::MySql>
        + sqlx::Type<sqlx::MySql>
        + sqlx::Decode<'r, sqlx::Postgres>
        + sqlx::Type<sqlx::Postgres>
{
}

/// A single row in a database table or query result that is used to
/// read values out of.
///
/// Wraps a SQLx row.
pub enum TableRow {
    MySql(sqlx::mysql::MySqlRow),
    Postgres(sqlx::postgres::PgRow),
}

impl DatabaseTypeAgnostic for TableRow {
    fn database_type(&self) -> DatabaseType {
        match self {
            Self::MySql(_) => DatabaseType::MySql,
            Self::Postgres(_) => DatabaseType::Postgres,
        }
    }
}

impl TableRow {
    /// Returns if the row is empty.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::MySql(row) => row.is_empty(),
            Self::Postgres(row) => row.is_empty(),
        }
    }

    /// Returns a vector of table column references, which can be used
    /// to fetch all data in the row.
    pub fn columns<'r>(&'r self) -> Vec<TableColumnRef<'r>> {
        match self {
            Self::MySql(row) => row.columns().iter().map(TableColumnRef::MySql).collect(),
            Self::Postgres(row) => row.columns().iter().map(TableColumnRef::Postgres).collect(),
        }
    }

    /// Utility method for returning the number of columns in the row
    /// without constructing an intermediate vector.
    pub fn column_count(&self) -> usize {
        match self {
            Self::MySql(row) => row.columns().len(),
            Self::Postgres(row) => row.columns().len(),
        }
    }

    /// Returns a reference to the table column for the given index.
    ///
    /// Panics on error.
    pub fn column<I>(&self, index: I) -> TableColumnRef
    where
        I: ColumnIndex,
    {
        self.try_column(index).unwrap()
    }

    /// Returns a reference to the table column for the given index.
    pub fn try_column<I>(&self, index: I) -> Result<TableColumnRef, Error>
    where
        I: ColumnIndex,
    {
        match self {
            Self::MySql(row) => match row.try_column(index) {
                Err(err) => Err(SqlxError::new(err)),
                Ok(column) => Ok(TableColumnRef::MySql(column)),
            },
            Self::Postgres(row) => match row.try_column(index) {
                Err(err) => Err(SqlxError::new(err)),
                Ok(column) => Ok(TableColumnRef::Postgres(column)),
            },
        }
    }

    /// Gets a reference to the value stored at the given column in the row.
    ///
    /// Panics on error.
    pub fn get<'r, T, I>(&'r self, index: I) -> T
    where
        T: ValueDecodable<'r>,
        I: ColumnIndex,
    {
        self.try_get(index).unwrap()
    }

    /// Gets a reference to the value stored at the given column in the row.
    pub fn try_get<'r, T, I>(&'r self, index: I) -> Result<T, Error>
    where
        T: ValueDecodable<'r>,
        I: ColumnIndex,
    {
        match self {
            Self::MySql(row) => match row.try_get_unchecked::<T, I>(index) {
                Err(err) => Err(SqlxError::new(err)),
                Ok(value) => Ok(value),
            },
            Self::Postgres(row) => match row.try_get_unchecked::<T, I>(index) {
                Err(err) => Err(SqlxError::new(err)),
                Ok(value) => Ok(value),
            },
        }
    }
}
