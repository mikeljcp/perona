use rusqlite::Connection;

pub(crate) struct PlayerQueries {
    pub client: Connection,
}

impl PlayerQueries {
    pub fn get_guild(&self, player: String) {
        let smt = self
            .client
            .prepare("SELECT guild FROM players WHERE name = $1");
    }
}
