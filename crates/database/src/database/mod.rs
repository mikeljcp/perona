mod connection;
mod db;

use connection::connection;

pub use db::Database;

/*
use std::sync::{LazyLock, Mutex, MutexGuard};
pub type DatabaseConnection = LazyLock<Mutex<Database>>;
pub type Connection<'a> = MutexGuard<'a, Database>;

pub static DATABASE: DatabaseConnection = LazyLock::new(|| Mutex::new(Database::new()));
*/