pub mod connection;
pub mod results;
pub mod utils;


//////////////////////////////////////////////////////////////////////////////////
//  Tests
//////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use crate::connection::Connection;

    use sqlx::{Row, Column};
    use futures::StreamExt;
    use std::ops::Deref;
    use sqlx::postgres::PgRow;
    use std::fmt::Pointer;
    use crate::utils::print_row;

    #[test]
    fn demo() {
        let settings = "postgres://postgres:password@localhost/datajoint";
        let mut con = Connection::new(settings.to_string());
        con.connect();
        let try_c = con.try_raw_query("select * from students where id = 0;");
        match try_c {
            Ok(mut cursor) => {
                let rows = cursor.fetch_all();
                for row in rows {
                    print_row(row)
                }
            },
            Err(e) =>{
                println!("{}",e)
            }
        }


    }
}