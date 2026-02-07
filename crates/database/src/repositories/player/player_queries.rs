use crate::database::Database;

pub struct PlayerQueries {
    pub client: Database,
}

impl PlayerQueries {
    pub fn new() -> Self {
        Self {
            client: Database::new(),
        }
    }

    pub fn insert_player(&self) -> bool {
        let a = "a";

        false
    }

    pub fn get_guild(&self, player: String) {
        let smt = self
            .client
            .prepare("SELECT guild FROM players WHERE name = $1");
    }
}
