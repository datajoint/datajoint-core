
use crate::results::TableRow;
use sqlx::TypeInfo;

pub fn format_row(row : TableRow) -> String {
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
            &_ => { println!("{}, {}", col.name(), col.to_owned().type_data.name() ) }
        }
    }
    format!("({})",form).to_string()
}


pub fn print_row_vec(row_vec: Vec<TableRow>){
    let mut form = "".to_string();
    for row in row_vec {
        form += &format_row(row).to_owned();
        form += "\n";
    }
    println!("{}",form);
}