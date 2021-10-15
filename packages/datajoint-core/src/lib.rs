#[macro_use]
extern crate num_derive;

pub mod connection;
pub mod results;
pub mod error;
pub mod types;
pub mod placeholders;
pub mod util;


//////////////////////////////////////////////////////////////////////////////////
//  Tests
//////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use crate::connection::{Connection, ConnectionSettings, DatabaseType};
    use crate::util::print_row_vec;
    use crate::placeholders::{PlaceHolderArgumentVector, PhArg, PlaceHolderArgument};
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
        let id = 1005;
        let mut args = PlaceHolderArgumentVector::new(vec![]);
        args.add(PlaceHolderArgument::new(DecodeResult::String("Temoc".to_string())));
        args.add(PlaceHolderArgument::new(DecodeResult::String("enarc".to_string())));
        args.add(PlaceHolderArgument::new(DecodeResult::Int32(id)));
        con.ph_execute_query("insert into students values ($1,$2,$3)", args);
        let mut args = PlaceHolderArgumentVector::new(vec![]);
        args.add(PlaceHolderArgument::new(DecodeResult::Int32(id)));
        let mut  try_c = con.ph_fetch_query("select * from students where id = $1;", args);
        let rows = try_c.rest();
        print_row_vec(rows);
    }

    #[test]
    fn demo_mysql() {
        // let settings = "mysql://username:password@tutorial-db.datajoint.io:3306/username_tutorial";
        // let mut con = Connection::new(settings.to_string());
        // con.connect().unwrap();
        //
        // let mut curse = con.raw_query("SELECT * FROM `edwardg_tutorial`.`mouse`");
        // let rows = curse.fetch_all();
        // let r= rows.len();
        // print!("{}",r);
        // print_row_vec(rows);
    }
}