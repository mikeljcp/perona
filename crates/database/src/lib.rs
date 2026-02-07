mod database;
mod repositories;

use repositories::Repositories;

pub fn repositories() -> Repositories {
    Repositories::new()
}