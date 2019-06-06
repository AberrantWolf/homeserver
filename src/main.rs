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

use self::homedb::models::{Log, NewLog};
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
fn get_logs(conn: LogsDbConn, log_id: i32) -> String {
    use schema::logs::dsl::{id, logs};
    let logs_list = logs.filter(id.eq(log_id)).load::<Log>(&*conn).expect("Unable to get logs or something");
    if logs_list.len() < 1 {
        return format!("No log found for id {}", log_id);
    }
    let log = logs_list.first().expect("There was no first");
    format!("Message Found: {}", log.msg)
}

#[get("/logs/write/<msg>")]
fn write_log(conn: LogsDbConn, msg: String) -> String {
    use schema::logs::dsl::{logs};

    let new_log = NewLog {msg: &msg};

    let result = diesel::insert_into(logs).values(&new_log).execute(&*conn);

    match result {
        Ok(num) => format!("Created a log {}", num),
        Err(err) => format!("Error {}", err),
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, hello, cookie, get_logs, write_log])
        .attach(LogsDbConn::fairing())
        .launch();
}
