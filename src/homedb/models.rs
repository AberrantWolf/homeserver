use rocket_contrib::databases::diesel::{AsChangeset, Insertable, Queryable};
use serde_derive::Serialize;

use crate::schema::{consoles, logs};

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Queryable, Serialize)]
pub struct Log {
    pub id: i32,
    pub msg: String,
}

#[derive(Insertable)]
#[table_name = "logs"]
pub struct NewLog<'a> {
    pub msg: &'a str,
}

// Consoles
