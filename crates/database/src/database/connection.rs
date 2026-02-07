use rusqlite::Connection;

pub fn connection() -> Result<Connection, String> {
    let conn = Connection::open("mu.db");

    if conn.is_err() {
        eprintln!("Not possible open file mu.db");
        return Err(String::new());
    }

    return Ok(conn.unwrap());
}
