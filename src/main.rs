use perona_database::Repositories;
fn main() {
    let db = Repositories::new();
    let _ = db.player_repository.set_player_rating("nicAa", 10);
}
