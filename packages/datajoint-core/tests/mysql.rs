use datajoint_core::{
    common::DatabaseType,
    connection::{Connection, ConnectionSettings},
    results::TableRow,
    types::NativeType,
};

#[test]
fn test_connection_to_db() {
    let mut settings = ConnectionSettings::new();
    settings.database_type = DatabaseType::MySql;
    settings.username = "root".to_string();
    settings.port = 1234;
    settings.password = "password".to_string();
    settings.database_name = "datajoint_core".to_string();
    settings.use_tls = Some(true);

    let mut conn = Connection::new(settings);
    let result = conn.connect();
    assert!(result.is_ok(), "Connection did not connect."); 
    let connected = conn.is_connected();
    assert!(connected, "Connection did not connect.");

    conn.disconnect();

    conn.settings.password = "wrongpassword".to_string();

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
fn test_insert_and_retrieve_multiple_rows() {
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
    con.execute_query("insert into tweet (text, owner_id) values ('hello world1', 5678);");
    con.execute_query("insert into tweet (text, owner_id) values ('hello world2', 9999);");
    con.execute_query("insert into tweet (text, owner_id) values ('hello world3', 3333);");
    con.execute_query("insert into tweet (text, owner_id) values ('hello world4', 123232321312);");
    let cursor = &mut con.try_fetch_query("select id, text, owner_id from tweet").unwrap();
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

#[test]
#[should_panic]
fn test_try_query_after_disconnect() {
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
    con.execute_query("delete from tweet where owner_id = 1234;");

    con.disconnect();

    con.execute_query("delete from tweet where owner_id = 1234;");
}

#[test]
#[should_panic]
fn test_postgres_query_to_mysql_db() {
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
    con.execute_query("select * from test where c in (select c from test2 where c<3 limit 1);");

    con.disconnect();
}

#[test]
fn run_test() {
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
