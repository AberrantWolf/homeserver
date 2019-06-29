use diesel::sqlite::SqliteConnection;
use rocket_contrib::database;

#[database("homedb")]
pub struct HomeDbConn(SqliteConnection);

pub mod companies;
pub mod consoles;
pub mod genres;
pub mod models;
