mod player;

use player::PlayerRepository;
pub struct Repositories {
    pub player_repository: PlayerRepository,
}

impl Repositories {
    pub fn new() -> Self {
        Self {
            player_repository: PlayerRepository::new(),
        }
    }
}
