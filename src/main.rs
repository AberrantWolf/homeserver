#![feature(proc_macro_hygiene, decl_macro, custom_attribute)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use diesel::prelude::*;

use rocket::http::{Cookie, Cookies};
use rocket_contrib::templates::Template;

mod homedb;
mod schema;

use homedb::HomeDbConn;

#[derive(Serialize)]
struct NullContext {}

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    cookie_msg: String,
}

#[get("/")]
fn index(cookies: Cookies) -> Template {
    let message_cookie = cookies.get("message");
    let cookie_message = if let Some(ref message) = message_cookie {
        print!("Cookies: {:?}", cookies);
        format!("Message: {}", message.value())
    } else {
        print!("Cookies: {:?}", cookies);
        "No cookie message".to_owned()
    };

    let context = TemplateContext {
        name: "Steve".to_owned(),
        cookie_msg: cookie_message,
    };
    Template::render("index", &context)
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
                homedb::companies::list_companies,
                homedb::companies::add_company,
                homedb::companies::update_company,
                homedb::companies::delete_company,
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
