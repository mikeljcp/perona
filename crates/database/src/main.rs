mod database;
mod migration;
mod repository;
pub use repository::PlayerRepository;
pub use database::Connection;

pub fn main() {
    migration::migration()
}