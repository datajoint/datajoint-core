use crate::common::{DatabaseType, DatabaseTypeAgnostic};
use crate::types::DataJointType;
use sqlx::{Column, TypeInfo};

/// Owned data about a table column.
pub struct TableColumn {
    pub database_type: DatabaseType,
    pub ordinal: usize,
    pub name: String,
    pub type_name: DataJointType,
}

/// Trait for types that can be used to index columns.
///
/// Currently implemented for string (column names) and numbers (ordinals).
pub trait ColumnIndex:
    sqlx::ColumnIndex<sqlx::mysql::MySqlRow> + sqlx::ColumnIndex<sqlx::postgres::PgRow>
{
}
impl<T> ColumnIndex for T where
    T: sqlx::ColumnIndex<sqlx::mysql::MySqlRow> + sqlx::ColumnIndex<sqlx::postgres::PgRow>
{
}

#[derive(Copy, Clone)]
pub enum TableColumnRef<'r> {
    MySql(&'r sqlx::mysql::MySqlColumn),
    Postgres(&'r sqlx::postgres::PgColumn),
}

impl<'r> DatabaseTypeAgnostic for TableColumnRef<'r> {
    fn database_type(&self) -> DatabaseType {
        match self {
            Self::MySql(_) => DatabaseType::MySql,
            Self::Postgres(_) => DatabaseType::Postgres,
        }
    }
}

/// A reference to data about a table column.
///
/// Wraps a SQLx column.
impl<'r> TableColumnRef<'r> {
    /// Returns the integer ordinal of the column, which can be used to
    /// fetch the column in a row.
    pub fn ordinal(&self) -> usize {
        match self {
            Self::MySql(column) => column.ordinal(),
            Self::Postgres(column) => column.ordinal(),
        }
    }

    /// Returns the name of the column, which can be used to fetch the
    /// column in a row.
    pub fn name(&self) -> &str {
        match self {
            Self::MySql(column) => column.name(),
            Self::Postgres(column) => column.name(),
        }
    }

    /// The DataJoint type for the column.
    pub fn type_name(&self) -> DataJointType {
        use DataJointType::*;
        match self {
            Self::MySql(column) => match column.type_info().name() {
                "BOOLEAN" => Boolean,
                "TINYINT" => TinyInt,
                "TINYINT UNSIGNED" => TinyIntUnsigned,
                "SMALLINT" => SmallInt,
                "SMALLINT UNSIGNED" => SmallIntUnsigned,
                "MEDIUMINT" => MediumInt,
                "MEDIUMINT UNSIGNED" => MediumIntUnsigned,
                "INT" => Int,
                "INT UNSIGNED" => IntUnsigned,
                "BIGINT" => BigInt,
                "BIGINT UNSIGNED" => BigIntUnsigned,
                "ENUM" => Enum,
                "DATE" => Date,
                "TIME" => Time,
                "DATETIME" => DateTime,
                "TIMESTAMP" => Timestamp,
                "CHAR" => CharN,
                "VARCHAR" => VarCharN,
                "FLOAT" => Float,
                "DOUBLE" => Double,
                "DECIMAL" => Decimal,
                "TINYBLOB" => TinyBlob,
                "MEDIUMBLOB" => MediumBlob,
                "BLOB" => Blob,
                "LONGBLOB" => LongBlob,
                "BINARY" => Binary,
                "VARBINARY" => Binary,
                &_ => Unknown,
            },
            Self::Postgres(column) => {
                let type_info = column.type_info();
                match type_info.kind() {
                    sqlx::postgres::PgTypeKind::Simple => match type_info.name() {
                        "BOOL" => Boolean,
                        "INT2" => SmallInt,
                        "INT4" => Int,
                        "INT8" => BigInt,
                        "CHAR" => CharN,
                        "VARCHAR" => VarCharN,
                        "TEXT" => VarCharN,
                        "BYTEA" => Binary,
                        "FLOAT4" => Float,
                        "FLOAT8" => Double,
                        "DATE" => Date,
                        "TIME" => Time,
                        "TIMESTAMP" => DateTime,
                        "TIMESTAMPTZ" => Timestamp,
                        "NUMERIC" => Decimal,
                        &_ => Unknown,
                        // TODO(jackson-nestelroad): Check all of the other Postgres types at
                        // https://docs.rs/sqlx-core/0.5.9/src/sqlx_core/postgres/type_info.rs.html#447.
                        //
                        // It is specifically interesting how arrays might be handled.
                    },
                    _ => Unknown,
                }
            }
        }
    }

    // Converts the column reference to an owned instance for storage.
    pub fn to_owned(&self) -> TableColumn {
        TableColumn {
            database_type: self.database_type(),
            ordinal: self.ordinal(),
            name: self.name().to_string(),
            type_name: self.type_name(),
        }
    }
}
