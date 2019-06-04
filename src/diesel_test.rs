#[macro_use]
extern crate diesel;

extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod homedb;

use homedb::models::Post;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("No DATABASE_URL set");
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    use self::schema::posts::dsl::*;

    let connection = establish_connection();

    let results = posts.filter(published.eq(true)).limit(5).load::<Post>(&connection).expect("Error fetching posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}