fn main() {
    let client = perona_memory::run(13104);

    match client {
        Ok(game) => {
            game.messages();
        }
        Err(err) => eprintln!("{}", err),
    }
}
