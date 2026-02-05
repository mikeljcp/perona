use rusqlite::Connection;

pub fn Connection() -> Connection {
    let conn = Connection::open("mu.db");

    if let Ok(conn) = conn {
        return conn;
    }

    panic!("")

}