use perona_database::Repositories;

use crate::constants::MIN_LEVEL;

mod constants;

pub struct PlayerValidation {
    pub killer: String,
    pub dead: String,
    pub repositories: Repositories,
}

impl PlayerValidation {
    pub fn new(killer: String, dead: String) -> Self {
        Self {
            killer,
            dead,
            repositories: Repositories::new(),
        }
    }

    pub fn level(&self, player_name: &str) -> bool {
        self.repositories.player_repository.get_level(player_name) > MIN_LEVEL
    }

    fn in_guild(&self, player_name: &str) -> bool {
        self.repositories
            .player_repository
            .get_guild(player_name)
            .is_some()
    }

    pub fn guild(&self) -> bool {
        let killer = &self.killer;
        let dead = &self.dead;

        if self.in_guild(killer) && self.in_guild(dead) {
            return self
                .repositories
                .player_repository
                .get_guild(killer)
                .unwrap()
                == self.repositories.player_repository.get_guild(dead).unwrap();
        }
        false
    }
}
