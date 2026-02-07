mod database;
mod repositories;

use repositories::Repositories;
use database::DATABASE;

pub fn a() -> Repositories {
    return Repositories::new()
}