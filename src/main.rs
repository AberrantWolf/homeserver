#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use diesel::prelude::*;

use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

mod homedb;
mod schema;

use self::homedb::models::{Log, NewLog};
use homedb::HomeDbConn;

#[derive(Serialize)]
struct NullContext {}

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<Log>,
    cookie_msg: String,
}

#[derive(FromForm)]
struct NewPostFormData {
    msg: String,
}

#[get("/")]
fn index(conn: HomeDbConn, cookies: Cookies) -> Template {
    let message_cookie = cookies.get("message");
    let cookie_message = if let Some(ref message) = message_cookie {
        print!("Cookies: {:?}", cookies);
        format!("Message: {}", message.value())
    } else {
        print!("Cookies: {:?}", cookies);
        "No cookie message".to_owned()
    };

    use schema::logs::dsl::{id, logs};
    let logs_list = logs
        .load::<Log>(&*conn)
        .expect("Unable to get logs or something");
    let items = logs_list;

    let context = TemplateContext {
        name: "Steve".to_owned(),
        items,
        cookie_msg: cookie_message,
    };
    Template::render("index", &context)
}

#[get("/new_post")]
fn new_post_page() -> Template {
    Template::render("new_post", NullContext {})
}

#[post("/new_post", data = "<form_data>")]
fn make_new_post(conn: HomeDbConn, form_data: Form<NewPostFormData>) -> Redirect {
    use schema::logs::dsl::logs;

    let msg = &form_data.msg;
    let new_log = NewLog { msg: &msg };
    let _result = diesel::insert_into(logs).values(&new_log).execute(&*conn);

    Redirect::to("/")
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
fn get_logs(conn: HomeDbConn, log_id: i32) -> String {
    use schema::logs::dsl::{id, logs};
    let logs_list = logs
        .filter(id.eq(log_id))
        .load::<Log>(&*conn)
        .expect("Unable to get logs or something");
    if logs_list.len() < 1 {
        return format!("No log found for id {}", log_id);
    }
    let log = logs_list.first().expect("There was no first");
    format!("Message Found: {}", log.msg)
}

#[get("/logs/write/<msg>")]
fn write_log(conn: HomeDbConn, msg: String) -> String {
    use schema::logs::dsl::logs;

    let new_log = NewLog { msg: &msg };
    let result = diesel::insert_into(logs).values(&new_log).execute(&*conn);

    match result {
        Ok(num) => format!("Created a log {}", num),
        Err(err) => format!("Error {}", err),
    }
}

//------- RETRO DB ROUTES ----------
// GAME CONSOLE

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                hello,
                cookie,
                get_logs,
                write_log,
                new_post_page,
                make_new_post,
                homedb::consoles::list_consoles,
                homedb::consoles::add_console,
                homedb::consoles::update_console,
                homedb::consoles::delete_console,
                homedb::genres::list_genres,
                homedb::genres::add_genre,
                homedb::genres::update_genre,
                homedb::genres::delete_genre,
            ],
        )
        .attach(HomeDbConn::fairing())
        .attach(Template::fairing())
        .launch();
}
