use datajoint_core::{
    common::DatabaseType,
    connection::{Connection, ConnectionSettings},
    results::TableRow,
    types::NativeType,
};

#[test]
fn test_connection_to_db() {
    let mut settings = ConnectionSettings::new();
    settings.database_type = DatabaseType::Postgres;
    settings.username = "postgres".to_string();
    settings.port = 5432;
    settings.password = "password".to_string();
    settings.database_name = "datajoint_core".to_string();
    settings.use_tls = Some(true);
    settings.hostname = "postgres_13".to_string();
    let mut conn = Connection::new(settings);

    let result = conn.connect();
    assert!(result.is_ok(), "Connection did not connect.");

    conn.disconnect();

    conn.settings.port = 1234;

    let result = conn.connect();
    assert!(result.is_err(), "Connection did not fail.");
}

#[test]
fn test_insert_and_retrieve_one_row() {
    let mut settings = ConnectionSettings::new();
    settings.database_type = DatabaseType::Postgres;
    settings.username = "postgres".to_string();
    settings.port = 5432;
    settings.password = "password".to_string();
    settings.database_name = "datajoint_core".to_string();
    settings.use_tls = Some(true);
    settings.hostname = "postgres_13".to_string();
    let mut con = Connection::new(settings);

    con.connect().unwrap();

    con.execute_query("TRUNCATE tweet");
    con.execute_query("insert into tweet (text, owner_id) values ('hello world', 1234);");
    let cursor = &mut con.fetch_query("select text, owner_id from tweet where owner_id=1234");
    let cursor = cursor;

    let rows: Vec<TableRow> = cursor.rest();
    let cols = rows[0].columns();

    let text = match rows[0].try_decode(cols[0]) {
        Ok(data) => data,
        Err(_) => NativeType::None,
    };

    let owner_id = match rows[0].try_decode(cols[1]) {
        Ok(data) => data,
        Err(_) => NativeType::None,
    };

    assert!(
        text == NativeType::String("hello world".to_string()),
        "text did not match \"hello world\"."
    );
    assert!(
        owner_id == NativeType::Int64(1234),
        "owner_id did not equal 1234."
    );
    con.disconnect();
}

#[test]
fn test_insert_and_retrieve_multiple_rows() {
    let mut settings = ConnectionSettings::new();
    settings.database_type = DatabaseType::Postgres;
    settings.username = "postgres".to_string();
    settings.port = 5432;
    settings.password = "password".to_string();
    settings.database_name = "datajoint_core".to_string();
    settings.use_tls = Some(true);
    settings.hostname = "postgres_13".to_string();
    let mut con = Connection::new(settings);

    con.connect().unwrap();

    con.execute_query("TRUNCATE tweet");
    con.execute_query("insert into tweet (text, owner_id) values ('hello world', 1234);");
    con.execute_query("insert into tweet (text, owner_id) values ('hello world1', 5678);");
    con.execute_query("insert into tweet (text, owner_id) values ('hello world2', 9999);");
    con.execute_query("insert into tweet (text, owner_id) values ('hello world3', 3333);");
    con.execute_query("insert into tweet (text, owner_id) values ('hello world4', 123232321312);");
    let cursor = &mut con
        .try_fetch_query("select id, text, owner_id from tweet")
        .unwrap();
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
fn test_placeholders() {
    let mut settings = ConnectionSettings::new();
    settings.database_type = DatabaseType::Postgres;
    settings.username = "postgres".to_string();
    settings.port = 5432;
    settings.password = "password".to_string();
    settings.database_name = "datajoint_core".to_string();
    settings.use_tls = Some(true);
    settings.hostname = "postgres_13".to_string();
    let mut con = Connection::new(settings);

    con.connect().unwrap();
    con.execute_query("TRUNCATE tweet");
    con.execute_query("insert into tweet (text, owner_id) values ('hello world', 1234);");

    let placeholders: Vec<NativeType> = vec![NativeType::Int32(1234)];
    let mut cursor = con.fetch_query_ph("select * from tweet where owner_id = $1;", placeholders);
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

    let placeholders: Vec<NativeType> = vec![NativeType::Int32(1234)];
    let rows_affected = con.execute_query_ph("delete from tweet where owner_id = $1", placeholders);

    con.disconnect()
}

#[test]
#[should_panic]
fn test_try_query_after_disconnect() {
    let mut settings = ConnectionSettings::new();
    settings.database_type = DatabaseType::Postgres;
    settings.username = "postgres".to_string();
    settings.port = 5432;
    settings.password = "password".to_string();
    settings.database_name = "datajoint_core".to_string();
    settings.use_tls = Some(true);
    settings.hostname = "postgres_13".to_string();
    let mut con = Connection::new(settings);

    con.connect().unwrap();

    con.execute_query("TRUNCATE tweet");
    con.execute_query("insert into tweet (text, owner_id) values ('hello world', 1234);");
    con.execute_query("delete from tweet where owner_id = 1234;");

    con.disconnect();

    con.execute_query("delete from tweet where owner_id = 1234;");
}

#[test]
fn test_executor() {
    let mut settings = ConnectionSettings::new();
    settings.database_type = DatabaseType::Postgres;
    settings.username = "postgres".to_string();
    settings.port = 5432;
    settings.password = "password".to_string();
    settings.database_name = "datajoint_core".to_string();
    settings.use_tls = Some(true);
    settings.hostname = "postgres_13".to_string();
    let mut con = Connection::new(settings);

    con.connect().unwrap();

    con.execute_query("TRUNCATE tweet");
    let executor = con.executor();

    // Execute a non-returning query.
    let rows_affected: u64 =
        executor.execute("insert into tweet (text, owner_id) values ('hello world', 1234);");
    assert!(rows_affected == 1, "Rows affected did not equal 1.");

    // Fetch a single row, ignoring the rest.
    let single_row: TableRow = executor.fetch_one("select * from tweet;");
    for col in single_row.columns() {
        match single_row.try_decode(col) {
            Ok(data) => {
                println!("{}", data)
            }
            Err(_) => {}
        }
    }

    // Testing executor with placeholders
    let placeholders: Vec<NativeType> = vec![
        NativeType::String("hello world".to_string()),
        NativeType::Int32(1234),
    ];
    let rows_affected = executor.execute_ph(
        "insert into tweet (text, owner_id) values ($1, $2);",
        placeholders,
    );
    assert!(rows_affected == 1, "Rows affected did not equal 1.");

    let placeholders: Vec<NativeType> = vec![NativeType::Int32(1234)];
    let cursor = &mut executor
        .cursor_ph("select * from tweet where owner_id = $1", placeholders)
        .unwrap();

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

    // Fetch all rows at once without a cursor.
    let all_rows: Vec<TableRow> = executor.fetch_all("select * from tweet;");
    for row in all_rows {
        for col in row.columns() {
            match row.try_decode(col) {
                Ok(data) => {
                    println!("{}", data)
                }
                Err(_) => {}
            }
        }
    }

    // Create a cursor for the given query.
    let cursor = executor.cursor("select * from tweet;");
    let cursor = &mut cursor.unwrap();

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
    con.disconnect();
}

#[test]
fn run_test() {
    let mut settings = ConnectionSettings::new();
    settings.database_type = DatabaseType::Postgres;
    settings.username = "postgres".to_string();
    settings.port = 5432;
    settings.password = "password".to_string();
    settings.database_name = "datajoint_core".to_string();
    settings.use_tls = Some(true);
    settings.hostname = "postgres_13".to_string();
    let mut con = Connection::new(settings);

    con.connect().unwrap();

    con.execute_query("TRUNCATE tweet");
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
