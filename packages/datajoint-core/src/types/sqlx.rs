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
            // For now, TIMESTAMP is broken in Postgres, but TIMESTAMPTZ is not.
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_from_sqlx_type_name() {
        let dj_type = DataJointType::from_sqlx_type_name("TINYINT");
        assert_eq!(dj_type, DataJointType::TinyInt);

        let dj_type = DataJointType::from_sqlx_type_name("TINYINT UNSIGNED");
        assert_eq!(dj_type, DataJointType::TinyIntUnsigned);

        let dj_type = DataJointType::from_sqlx_type_name("SMALLINT");
        assert_eq!(dj_type, DataJointType::SmallInt);

        let dj_type = DataJointType::from_sqlx_type_name("SMALLINT UNSIGNED");
        assert_eq!(dj_type, DataJointType::SmallIntUnsigned);

        let dj_type = DataJointType::from_sqlx_type_name("MEDIUMINT");
        assert_eq!(dj_type, DataJointType::MediumInt);

        let dj_type = DataJointType::from_sqlx_type_name("MEDIUMINT UNSIGNED");
        assert_eq!(dj_type, DataJointType::MediumIntUnsigned);

        let dj_type = DataJointType::from_sqlx_type_name("INT");
        assert_eq!(dj_type, DataJointType::Int);

        let dj_type = DataJointType::from_sqlx_type_name("INT UNSIGNED");
        assert_eq!(dj_type, DataJointType::IntUnsigned);

        let dj_type = DataJointType::from_sqlx_type_name("ENUM");
        assert_eq!(dj_type, DataJointType::Enum);

        let dj_type = DataJointType::from_sqlx_type_name("DATE");
        assert_eq!(dj_type, DataJointType::Date);

        let dj_type = DataJointType::from_sqlx_type_name("TIME");
        assert_eq!(dj_type, DataJointType::Time);

        let dj_type = DataJointType::from_sqlx_type_name("DATETIME");
        assert_eq!(dj_type, DataJointType::DateTime);

        let dj_type = DataJointType::from_sqlx_type_name("TIMESTAMP");
        assert_eq!(dj_type, DataJointType::Timestamp);

        let dj_type = DataJointType::from_sqlx_type_name("CHAR");
        assert_eq!(dj_type, DataJointType::CharN);

        let dj_type = DataJointType::from_sqlx_type_name("VARCHAR");
        assert_eq!(dj_type, DataJointType::VarCharN);
        
        let dj_type = DataJointType::from_sqlx_type_name("FLOAT");
        assert_eq!(dj_type, DataJointType::Float);

        let dj_type = DataJointType::from_sqlx_type_name("DOUBLE");
        assert_eq!(dj_type, DataJointType::Double);

        let dj_type = DataJointType::from_sqlx_type_name("DECIMAL");
        assert_eq!(dj_type, DataJointType::Decimal);

        let dj_type = DataJointType::from_sqlx_type_name("TINYBLOB");
        assert_eq!(dj_type, DataJointType::TinyBlob);

        let dj_type = DataJointType::from_sqlx_type_name("MEDIUMBLOB");
        assert_eq!(dj_type, DataJointType::MediumBlob);

        let dj_type = DataJointType::from_sqlx_type_name("BLOB");
        assert_eq!(dj_type, DataJointType::Blob);

        let dj_type = DataJointType::from_sqlx_type_name("LONGBLOB");
        assert_eq!(dj_type, DataJointType::LongBlob);

        let dj_type = DataJointType::from_sqlx_type_name("INT2");
        assert_eq!(dj_type, DataJointType::SmallInt);

        let dj_type = DataJointType::from_sqlx_type_name("INT4");
        assert_eq!(dj_type, DataJointType::Int);

        let dj_type = DataJointType::from_sqlx_type_name("TEXT");
        assert_eq!(dj_type, DataJointType::VarCharN);

        let dj_type = DataJointType::from_sqlx_type_name("FLOAT4");
        assert_eq!(dj_type, DataJointType::Float);

        let dj_type = DataJointType::from_sqlx_type_name("FLOAT8");
        assert_eq!(dj_type, DataJointType::Double);

        let dj_type = DataJointType::from_sqlx_type_name("TIMESTAMPTZ");
        assert_eq!(dj_type, DataJointType::Timestamp);
    }
}
