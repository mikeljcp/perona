fn main() {
    let client = perona_memory::read(4684);

    match client {
        Ok(game) => {
         println!("{:?}", game.messages());
      }
        Err(err) => eprintln!("{}", err),
    }
}
