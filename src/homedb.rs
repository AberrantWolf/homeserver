#![feature(proc_macro_hygiene, decl_macro)]

use diesel::sqlite::SqliteConnection;

#[database("homedb")]
pub struct HomeDbConn(SqliteConnection);

pub mod models;
pub mod consoles;