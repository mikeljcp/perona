use perona_types::Player;

use crate::database::Database;

pub struct PlayerRepository {
    database: Database,
}

impl PlayerRepository {
    pub fn new() -> Self {
        Self {
            database: Database::new(),
        }
    }

    pub fn insert_player(&self, player: Player) {
        let Player {
            name, level, guild, ..
        } = player;
        let result = self.database.execute(
            "INSERT INTO (name, level, guild) VALUES ($1, $2, $3)",
            (name, level, guild),
        );

        println!("{result:?}");
    }

    /*
    pub fn get_guild(&self, player: &String) -> Option<String> {
        Some(String::new())
    }

    pub fn get_level(&self, player: String) -> i32 {
        0
    }

    pub fn get_player_rating(&self, player: String) {}

    pub fn get_last_kill(&self, player: String) {}

    pub fn get_kills_count(&self, player: String) {}

    pub fn get_players_killed(&self, player: String) {}
    */
}
