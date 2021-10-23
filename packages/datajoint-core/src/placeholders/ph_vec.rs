use crate::types::NativeType;

/// A SQLx query bound to zero or more placeholder arguments.
pub type SqlxQuery<'q> =
    sqlx::query::Query<'q, sqlx::Any, <sqlx::Any as sqlx::database::HasArguments<'q>>::Arguments>;

/// A type trait for binding any amount of placeholder arguments to a query.
pub trait PlaceholderArgumentCollection {
    fn prepare(self, query: &str) -> SqlxQuery;
}

/// A single placeholder argument.
pub type PlaceholderArgument = NativeType;

/// A basic placeholder argument vector, which wraps several values of a supported native type.
pub type PlaceholderArgumentVector = Vec<PlaceholderArgument>;

impl PlaceholderArgumentCollection for PlaceholderArgumentVector {
    fn prepare(self, query: &str) -> SqlxQuery {
        let mut query = sqlx::query::<sqlx::Any>(query);
        for arg in self {
            match arg {
                NativeType::None => {}
                NativeType::Int8(val) => query = query.bind(val as i32),
                NativeType::UInt8(val) => query = query.bind(val as i32),
                NativeType::Int16(val) => query = query.bind(val as i32),
                NativeType::UInt16(val) => query = query.bind(val as i32),
                NativeType::Int32(val) => query = query.bind(val),
                // TODO(EdwardGarmon): Will eventually move to using
                // sqlx type parameters so we can encode types correctly
                // according to database type, for now there
                // will be a possible overflow error here.
                NativeType::UInt32(val) => query = query.bind(val as i32),
                NativeType::String(val) => query = query.bind(val),
                NativeType::Float32(val) => query = query.bind(val),
                NativeType::Float64(val) => query = query.bind(val),
                NativeType::Bytes(val) => query = query.bind(val),
            };
        }
        query
    }
}
