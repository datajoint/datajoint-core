pub mod connection;
pub mod results;
pub mod utils;
pub mod error;
pub mod types;
pub mod placeholders;


//////////////////////////////////////////////////////////////////////////////////
//  Tests
//////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use crate::connection::{Connection};
    use crate::utils::print_row_vec;
    use crate::placeholders::{PlaceHolderArgumentVector, PhArg, PlaceHolderArgument};
    use crate::types::DecodeResult;

    #[test]
    fn demo_postgres() {

        let settings = "postgres://postgres:password@localhost/datajoint";
        let mut con = Connection::new(settings.to_string());
        con.connect().unwrap();
        let id = 1003;
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
        con.disconnect();

        // this should fail now that the connection is closed
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
