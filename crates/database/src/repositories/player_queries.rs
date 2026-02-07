use rusqlite::Connection;

pub struct PlayerQueries {
    pub client: Connection,
}

impl PlayerQueries {

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
