pub mod connection;
pub mod results;
pub mod utils;

//////////////////////////////////////////////////////////////////////////////////
//  Tests
//////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use crate::connection::Connection;
    use crate::utils::print_row_vec;

    #[test]
    fn demo_postgres() {
        let settings = "postgres://postgres:password@localhost/datajoint";
        let mut con = Connection::new(settings.to_string());
        con.connect().unwrap();
        let mut  try_c = con.raw_query("select * from students where id = 0;");
        let rows = try_c.fetch_all();
        print_row_vec(rows);
    }

    #[test]
    fn demo_mysql() {
        let settings = "mysql://username:password@tutorial-db.datajoint.io:3306/username_tutorial";
        let mut con = Connection::new(settings.to_string());
        con.connect().unwrap();

        let mut curse = con.raw_query("SELECT * FROM `edwardg_tutorial`.`mouse`");
        let rows = curse.fetch_all();
        let r= rows.len();
        print!("{}",r);
        print_row_vec(rows);

    }
}
