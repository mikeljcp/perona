use perona_types::Player;

use crate::repositories::player::PlayerQueries;

pub struct PlayerRepository {
    query: PlayerQueries,
}

impl PlayerRepository {
    pub fn new() -> Self {
        Self {
            query: PlayerQueries::new(),
        }
    }

    pub fn add_player(&self, player_name: &str, guild: &str) -> bool {
        self.query.add_player(player_name, guild)
    }

    pub fn get_player(&self, player_name: &str) -> Option<Player> {
        self.query.get_player(player_name)
    }

    pub fn get_guild(&self, player_name: &str) -> Option<String> {
        self.query.get_guild(player_name)
    }

    pub fn get_level(&self, player_name: &str) -> i64 {
        self.query.get_level(player_name)
    }

    pub fn get_player_rating(&self, player_name: &str) -> i64 {
        self.query.get_player_rating(player_name)
    }

    pub fn get_last_kill(&self, player_name: &str) -> Option<String> {
        self.query.get_last_kill(player_name)
    }

    pub fn set_player_rating(&self, player_name: &str, player_rating: i64) -> Result<bool, bool> {
        self.query.set_player_rating(player_name, player_rating)
    }
}
