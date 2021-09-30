
use crate::results::TableRow;
use sqlx::TypeInfo;

pub fn print_row(row : TableRow){
    let cols = row.columns();
    let mut form = "".to_string();
    for col in cols {
        match col.to_owned().type_data.name() {
           "VARCHAR" => {
               let str : &str = row.get(col.name());
               form += &*format!("{}, ", str);
           },
           "INT4" =>{
               let i: i32 = row.get(col.name());
               form += &*format!("{}, ", i);
            }
            &_ => {}
        }
    }
    println!("({})",form);
}
