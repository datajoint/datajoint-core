/// Enum type for representing the type of SQL database to connect to.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(i32)]
pub enum DatabaseType {
    MySql,
    Postgres,
}

/// Trait for types that are database type agnostic, which means they wrap
/// some database-specific types to provide a common API.
///
/// More of a convenience method than anything. This method should not be used
/// internally when enum variants can be used instead.
pub trait DatabaseTypeAgnostic {
    fn database_type(&self) -> DatabaseType;
}
