use crate::types::DataJointType;

impl DataJointType {
    /// Maps a SQLx type, currently only exposed as a string, to a
    /// supported DataJoint type.
    ///
    /// Currently only supports MySQL and Postgres type names given
    /// by SQLx.
    pub fn from_sqlx_type_name(name: &str) -> DataJointType {
        use DataJointType::*;
        match name {
            // MySQL type names.
            "TINYINT" => TinyInt,
            "TINYINT UNSIGNED" => TinyIntUnsigned,
            "SMALLINT" => SmallInt,
            "SMALLINT UNSIGNED" => SmallIntUnsigned,
            "MEDIUMINT" => MediumInt,
            "MEDIUMINT UNSIGNED" => MediumIntUnsigned,
            "INT" => Int,
            "INT UNSIGNED" => IntUnsigned,
            "ENUM" => Enum,
            "DATE" => Date,
            "TIME" => Time,
            "DATETIME" => DateTime,
            // MySQL considers timestamps to have a timezone, Postgres does not.
            // This is a major problem that currently cannot be resolved without
            // providing the database type stored in ConnectionSettings to
            // TableColumnRef::from_sqlx_type_name.
            //
            // For now, Timestamp is broken in MySQL.
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

            // Postgres-specific type names.
            "INT2" => SmallInt,
            "INT4" => Int,
            "TEXT" => VarCharN,
            "FLOAT4" => Float,
            "FLOAT8" => Double,
            "TIMESTAMPTZ" => Timestamp,

            &_ => Unknown,
        }
    }
}
