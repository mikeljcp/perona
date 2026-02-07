use perona_database::Repositories;
use perona_types::Player;

fn main() {
    let db = Repositories::new();

    let mock_player = Player {
        name: todo!(),
        level: todo!(),
        guild: todo!(),
        last_kill: todo!(),
        killst_count: todo!(),
        in_md3: todo!(),
        elo: todo!(),
        division: todo!(),
        player_rating: todo!(),
    };

    db.player_repository.insert_player(mock_player);
}
