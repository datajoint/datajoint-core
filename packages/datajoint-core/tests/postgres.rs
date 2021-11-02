use datajoint_core::{
    common::DatabaseType,
    connection::{Connection, ConnectionSettings},
    results::TableRow,
};

#[test]
fn run_test() {
    let mut settings = ConnectionSettings::new();
    settings.database_type = DatabaseType::Postgres;
    settings.username = "postgres".to_string();
    settings.port = 5432;
    settings.password = "password".to_string();
    settings.database_name = "postgres".to_string();
    settings.use_tls = Some(true);
    let mut con = Connection::new(settings);
    con.connect().unwrap();

    con.execute_query("truncate tweet");
    con.execute_query("insert into tweet (text, owner_id) values ('hello world', 1234);");

    let cursor = &mut con.fetch_query("select * from tweet");
    let cursor = cursor;
    // TODO(EdwardGarmon): remove unsafe block that is necessary due to pinned cursor
    let rows: Vec<TableRow> = unsafe {
        match std::pin::Pin::as_mut(cursor).get_unchecked_mut().try_rest() {
            Ok(value) => value,
            Err(err) => {
                println!("{} error unsafely accessing pinned data", err);
                panic!()
            }
        }
    };

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
