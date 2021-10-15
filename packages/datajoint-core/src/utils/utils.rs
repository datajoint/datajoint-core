
use crate::results::TableRow;
use sqlx::TypeInfo;
use crate::types::DataJointType;

pub fn format_row(row : TableRow) -> String {
    let cols = row.columns();
    let mut form = "".to_string();
    for col in cols {

        // TODO implement type decoding here
        match col.type_name() {
            DataJointType::Unknown => {}
            DataJointType::TinyInt => {}
            DataJointType::TinyIntUnsigned => {}
            DataJointType::SmallInt => {}
            DataJointType::SmallIntUnsigned => {}
            DataJointType::MediumInt => {}
            DataJointType::MediumIntUnsigned => {}
            DataJointType::Int => {}
            DataJointType::IntUnsigned => {}
            DataJointType::Enum => {}
            DataJointType::Date => {}
            DataJointType::Time => {}
            DataJointType::DateTime => {}
            DataJointType::Timestamp => {}
            DataJointType::CharN => {}
            DataJointType::VarCharN => {}
            DataJointType::Float => {}
            DataJointType::Double => {}
            DataJointType::Decimal => {}
            DataJointType::TinyBlob => {}
            DataJointType::MediumBlob => {}
            DataJointType::Blob => {}
            DataJointType::LongBlob => {}
            DataJointType::Attach => {}
            DataJointType::FilepathStore => {}
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