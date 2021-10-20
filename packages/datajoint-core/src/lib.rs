#[macro_use]
extern crate num_derive;

pub mod connection;
pub mod error;
pub mod placeholders;
pub mod results;
pub mod types;
pub mod util;

//////////////////////////////////////////////////////////////////////////////////
//  Tests
//////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use crate::connection::{Connection, ConnectionSettings, DatabaseType};
    use crate::placeholders::{PlaceholderArgument, PlaceholderArgumentVector};
    use crate::types::DecodeResult;

    #[test]
    fn demo_postgres() {
        let mut settings = ConnectionSettings::new();
        settings.password = "password".to_string();
        settings.database_name = "datajoint".to_string();
        settings.hostname = "localhost".to_string();
        settings.username = "postgres".to_string();
        settings.database_type = DatabaseType::Postgres;
        settings.port = 5432;
        settings.use_tls = Some(true);

        let mut con = Connection::new(settings);
        con.connect().unwrap();
        let id = 1011;
        let mut args = PlaceholderArgumentVector::new(vec![]);
        args.add_arg(PlaceholderArgument::new(DecodeResult::String(
            "Temoc".to_string(),
        )));
        args.add_arg(PlaceholderArgument::new(DecodeResult::String(
            "enarc".to_string(),
        )));
        args.add_arg(PlaceholderArgument::new(DecodeResult::Int32(id)));
        con.execute_query_ph("insert into students values ($1,$2,$3)", args);
        let mut args = PlaceholderArgumentVector::new(vec![]);
        args.add_arg(PlaceholderArgument::new(DecodeResult::Int32(id)));
        let mut try_c = con.fetch_query_ph("select * from students where id = $1;", args);
        let _rows = try_c.rest();
    }

    #[test]
    fn demo_mysql() {}
}
