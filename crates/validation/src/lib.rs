//use perona_database::PlayerRepository;

use crate::constants::MIN_LEVEL;

mod constants;

pub struct PlayerValidation {
    pub killer: String,
    pub dead: String,
}

impl PlayerValidation {
    
    /*
    pub fn new(killer: String, dead: String) -> Self {
        Self { killer, dead }
    }

    pub fn level(&self, player: String) -> bool {
        PlayerRepository.get_level(player) > MIN_LEVEL
    }

    fn in_guild(&self, player: &String) -> bool {
        PlayerRepository.get_guild(player).is_some()
    }

    pub fn guild(&self) -> bool {
        let killer = &self.killer;
        let dead = &self.dead;

        if self.in_guild(killer) && self.in_guild(dead) {
            return PlayerRepository.get_guild(killer).unwrap()
                == PlayerRepository.get_guild(dead).unwrap();
        }
        false
    }
    */
}
