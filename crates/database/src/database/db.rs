use rusqlite::{Connection, Params, Statement};

use crate::database::connection;

pub struct Database {
    client: Connection,
}

impl Database {
    pub fn new() -> Self {
        let conn = connection().expect("Error to connect in database");

        Self { client: conn }
    }

    pub fn execute<P>(&self, query: &str, params: P) -> Result<usize, rusqlite::Error>
    where
        P: Params,
    {
        self.client.execute(query, params)
    }

    pub fn prepare(&self, query: &str) -> Result<Statement<'_>, rusqlite::Error> {
        self.client.prepare(query)
    }
}
