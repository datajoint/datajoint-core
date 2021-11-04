use datajoint_core::{
    common::DatabaseType,
    connection::{Connection, ConnectionSettings},
    results::TableRow,
    types::NativeType
};

#[test]
fn test_connection_to_db() {
    let mut settings = ConnectionSettings::new();
    settings.database_type = DatabaseType::Postgres;
    settings.username = "postgres".to_string();
    settings.port = 5678;
    settings.password = "password".to_string();
    settings.database_name = "postgres".to_string();
    settings.use_tls = Some(true);

    let mut conn = Connection::new(settings);
    let result = conn.connect();
    assert!(result.is_ok(), "Connection did not connect."); 

    conn.disconnect();

    conn.settings.port = 5432;

    let result = conn.connect();
    assert!(result.is_err(), "Connection did not fail.");
}

#[test]
fn test_insert_and_retrieve_one_row() {
    let mut settings = ConnectionSettings::new();
    settings.database_type = DatabaseType::MySql;
    settings.username = "root".to_string();
    settings.port = 1234;
    settings.password = "password".to_string();
    settings.database_name = "datajoint_core".to_string();
    settings.use_tls = Some(true);
    let mut con = Connection::new(settings);

    con.connect().unwrap();

    con.execute_query("truncate tweet");
    con.execute_query("insert into tweet (text, owner_id) values ('hello world', 1234);");
    let cursor = &mut con.fetch_query("select id, text, owner_id from tweet limit 1");
    let cursor = cursor;

    let rows: Vec<TableRow> = cursor.rest();
    let cols = rows[0].columns();

    let id = match rows[0].try_decode(cols[0]) {
        Ok(data) => { data }
        Err(_) => {NativeType::None}
    };

    let text = match rows[0].try_decode(cols[1]) {
        Ok(data) => { data }
        Err(_) => {NativeType::None}
    };

    let owner_id = match rows[0].try_decode(cols[2]) {
        Ok(data) => { data }
        Err(_) => {NativeType::None}
    };

    assert!(id == NativeType::Int64(1), "id did not equal 1.");
    assert!(text == NativeType::String("hello world".to_string()), "text did not match \"hello world\".");
    assert!(owner_id == NativeType::Int64(1234), "owner_id did not equal 1234.");
}


#[test]
fn run_test() {
    let mut settings = ConnectionSettings::new();
    settings.database_type = DatabaseType::Postgres;
    settings.username = "postgres".to_string();
    settings.port = 5678;
    settings.password = "password".to_string();
    settings.database_name = "postgres".to_string();
    settings.use_tls = Some(true);
    let mut con = Connection::new(settings);
    con.connect().unwrap();

    con.execute_query("truncate tweet");
    con.execute_query("insert into tweet (text, owner_id) values ('hello world', 1234);");

    let cursor = &mut con.fetch_query("select * from tweet");
    let cursor = cursor;

    let rows: Vec<TableRow> = cursor.rest();
    for row in rows {
        for col in row.columns() {
            match row.try_decode(col) {
                Ok(data) => {
                    println!("{}", data)
                }
                Err(_) => {}
            }
        }
    }

    con.disconnect()
}
