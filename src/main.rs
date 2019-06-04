#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use rocket::http::{Cookie, Cookies};
// use rocket_contrib::databases::diesel;

mod homedb;
mod schema;

use self::homedb::models::Log;
use schema::*;

#[database("homedb")]
struct LogsDbConn(SqliteConnection);

#[get("/")]
fn index(cookies: Cookies) -> String {
    let message_cookie = cookies.get("message");
    if let Some(ref message) = message_cookie {
        print!("Cookies: {:?}", cookies);
        return format!("Message: {}", message.value());
    } else {
        print!("Cookies: {:?}", cookies);
    }
    "No message saved".to_owned()
}

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}!", name.as_str())
}

#[get("/cookie/<msg>")]
fn cookie(msg: String, mut cookies: Cookies) -> String {
    let simple = Cookie::new("simple", msg.clone());
    println!("Simple Cookie: {:?}", simple);
    cookies.add(simple);
    let cookie = Cookie::build("message", msg)
        .path("/")
        .secure(false)
        .finish();
    cookies.add(cookie);
    "Cookie message saved!".to_owned()
}

#[get("/logs/<log_id>")]
fn get_logs(conn: LogsDbConn, log_id: usize) -> Result<Log> {
    use schema::logs::dsl::*;
    // logs.filter(id.eq(log_id)).load::<Log>(&conn)?
    logs.first(&*conn)?
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, hello, cookie, get_logs])
        .attach(LogsDbConn::fairing())
        .launch();
}
