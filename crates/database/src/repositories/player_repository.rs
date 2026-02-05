pub struct PlayerRepository;

impl PlayerRepository {
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
}
