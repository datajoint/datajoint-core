use crate::error::{DataJointError, Error, ErrorCode};
use crate::query::Query;
use crate::types::NativeType;

/// A type trait for binding any amount of placeholder arguments to a query.
pub trait PlaceholderArgumentCollection {
    /// Binds the placeholder arguments to the given query.
    ///
    /// Returns the new query with the bound parameters.
    fn bind_to_query<'q>(self, query: Query<'q>) -> Result<Query<'q>, Error>;
}

/// A single placeholder argument.
pub type PlaceholderArgument = NativeType;

/// A basic placeholder argument vector, which wraps several values of a supported native type.
pub type PlaceholderArgumentVector = Vec<PlaceholderArgument>;

impl PlaceholderArgumentCollection for PlaceholderArgumentVector {
    fn bind_to_query<'q>(self, query: Query<'q>) -> Result<Query<'q>, Error> {
        match query {
            Query::MySql(mut query) => {
                for arg in self {
                    match arg {
                        NativeType::None => {}
                        NativeType::Bool(val) => query = query.bind(val),
                        NativeType::Int8(val) => query = query.bind(val),
                        NativeType::UInt8(val) => query = query.bind(val),
                        NativeType::Int16(val) => query = query.bind(val),
                        NativeType::UInt16(val) => query = query.bind(val),
                        NativeType::Int32(val) => query = query.bind(val),
                        NativeType::UInt32(val) => query = query.bind(val),
                        NativeType::Int64(val) => query = query.bind(val),
                        NativeType::UInt64(val) => query = query.bind(val),
                        NativeType::String(val) => query = query.bind(val),
                        NativeType::Float32(val) => query = query.bind(val),
                        NativeType::Float64(val) => query = query.bind(val),
                        NativeType::Bytes(val) => query = query.bind(val),
                    };
                }
                Ok(Query::MySql(query))
            }
            Query::Postgres(mut query) => {
                for arg in self {
                    match arg {
                        NativeType::None => {}
                        NativeType::Bool(val) => query = query.bind(val),
                        NativeType::Int8(val) => query = query.bind(val),
                        NativeType::Int16(val) => query = query.bind(val),
                        NativeType::Int32(val) => query = query.bind(val),
                        NativeType::Int64(val) => query = query.bind(val),
                        NativeType::String(val) => query = query.bind(val),
                        NativeType::Float32(val) => query = query.bind(val),
                        NativeType::Float64(val) => query = query.bind(val),
                        NativeType::Bytes(val) => query = query.bind(val),
                        NativeType::UInt8(_)
                        | NativeType::UInt16(_)
                        | NativeType::UInt32(_)
                        | NativeType::UInt64(_) => {
                            return Err(DataJointError::new_with_message(
                                "postgres does not supported unsigned data types",
                                ErrorCode::UnsupportedNativeType,
                            ))
                        }
                    };
                }
                Ok(Query::Postgres(query))
            }
        }
    }
}
