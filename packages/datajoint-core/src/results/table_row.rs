use crate::results::table_column::{ColumnIndex, TableColumnRef};
use sqlx::Row;

/// Type trait for indicating if a type is safe to be decoded to.
///
/// Currently only implements how SQLx type checks return types.
pub trait ValueDecodable<'r>:
    sqlx::Decode<'r, sqlx::any::Any> + sqlx::Type<sqlx::any::Any>
{

}

impl<'r, T> ValueDecodable<'r> for T where
    T: sqlx::Decode<'r, sqlx::any::Any> + sqlx::Type<sqlx::any::Any>
{

}

/// A single row in a database table or query result that is used to
/// read values out of.
///
/// Wraps `sqlx::any::AnyRow`.
pub struct TableRow {
    row: sqlx::any::AnyRow,
}

impl TableRow {
    /// Creates a new table row around a SQLx row.
    pub fn new(row: sqlx::any::AnyRow) -> Self {
        return TableRow { row: row };
    }

    /// Returns if the row is empty.
    pub fn is_empty(&self) -> bool {
        self.row.is_empty()
    }

    /// Returns a vector of table column references, which can be used
    /// to fetch all data in the row.
    pub fn columns(&self) -> Vec<TableColumnRef> {
        self.row.columns().iter().map(TableColumnRef::new).collect()
    }

    /// Utility method for returning the number of columns in the row
    /// without constructing an intermediate vector.
    pub fn column_count(&self) -> usize {
        self.row.columns().len()
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
    pub fn try_column<I>(&self, index: I) -> Result<TableColumnRef, &str>
    where
        I: ColumnIndex,
    {
        match self.row.try_column(index) {
            Err(_) => Err("error in column"),
            Ok(column) => Ok(TableColumnRef::new(column)),
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
    pub fn try_get<'r, T, I>(&'r self, index: I) -> Result<T, &str>
    where
        T: ValueDecodable<'r>,
        I: ColumnIndex,
    {
        let result: Result<T, sqlx::Error> = self.row.try_get(index);
        match result {
            Err(_) => Err("error in get"),
            Ok(value) => Ok(value),
        }
    }
}
