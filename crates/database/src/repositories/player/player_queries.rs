use perona_types::{Player, constants::STANDARD_PR};
use rusqlite::params;

use crate::database::Database;

pub struct PlayerQueries {
    pub database: Database,
}

impl PlayerQueries {
    pub fn new() -> Self {
        Self {
            database: Database::new(),
        }
    }

    pub fn add_player(&self, player_name: &str, guild: &str) -> bool {
        self.database
            .execute(
                "INSERT INTO players (name, pr, guild) VALUES (?1, ?2, ?3)",
                (player_name, STANDARD_PR, guild),
            )
            .map_or(false, |row| row > 0)
    }

    pub fn get_player(&self, player_name: &str) -> Option<Player> {
        let result = self
            .database
            .prepare("SELECT * FROM players WHERE name = :playerName");

        if let Ok(mut stmt) = result {
            let rows = stmt.query_map(&[(":playerName", player_name)], |row| {
                Ok(Player {
                    name: row.get_unwrap(1),
                    level: row.get_unwrap(2),
                    guild: row.get_unwrap(3),
                    last_kill: row.get_unwrap(4),
                    kills_count: row.get_unwrap(5),
                    in_md3: row.get_unwrap(6),
                    elo: row.get_unwrap(7),
                    division: row.get_unwrap(8),
                    player_rating: row.get_unwrap(9),
                })
            });

            if let Ok(row) = rows {
                let mut players: Vec<Player> = Vec::new();

                for player in row {
                    if let Ok(player) = player {
                        players.push(player);
                    }
                }

                if players.len() == 0 {
                    return None;
                }

                let get_player = players.first().unwrap().clone();

                return Some(get_player);
            }

            return None;
        }

        None
    }

    pub fn get_guild(&self, player_name: &str) -> Option<String> {
        self.get_player(player_name).map(|player| player.guild)
    }

    pub fn get_level(&self, player_name: &str) -> i64 {
        self.get_player(player_name)
            .map_or(1, |player| player.level)
    }

    pub fn get_player_rating(&self, player_name: &str) -> i64 {
        self.get_player(player_name)
            .map_or(0, |player| player.player_rating)
    }

    pub fn get_last_kill(&self, player_name: &str) -> Option<String> {
        self.get_player(player_name)
            .map(|player| player.last_kill.unwrap())
    }

    pub fn set_player_rating(&self, player_name: &str, player_rating: i64) -> Result<bool, bool> {
        self.database
            .execute(
                "UPDATE players SET pr = ?1 WHERE name = ?2",
                params![player_rating, player_name],
            )
            .map_or(Err(false), |row| Ok(row > 0))
    }
}
