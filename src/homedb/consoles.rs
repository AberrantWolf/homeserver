use rocket;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib;
use rocket_contrib::databases::diesel::{AsChangeset, Insertable, Queryable};
use rocket_contrib::templates::Template;
use serde_derive::Serialize;

use super::HomeDbConn;
use crate::schema::consoles;
use crate::*;

#[derive(Queryable, Serialize)]
pub struct Console {
    pub _id: i32,
    pub short_name: String,
    pub long_name: String,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "consoles"]
pub struct ConsoleTransport<'a> {
    pub _id: Option<i32>,
    pub short_name: &'a str,
    pub long_name: &'a str,
}

#[derive(Serialize)]
pub struct ConsoleTemplateContext {
    err_msg: Option<String>,
    consoles_list: Vec<Console>,
}

#[derive(FromForm, Debug)]
pub struct ConsoleFormData {
    id: Option<i32>,
    short_name: String,
    long_name: String,
}

// ----- BINDINGS -----

#[get("/consoles")]
pub fn list_consoles(conn: HomeDbConn) -> Template {
    use schema::consoles::dsl::consoles;

    let consoles_list_result = consoles.load::<Console>(&*conn);
    let (err_msg, consoles_list) = match consoles_list_result {
        Ok(list) => (None, list),
        Err(e) => (Some("Unable to get consoles list :(".to_owned()), vec![]),
    };

    let context = ConsoleTemplateContext {
        err_msg,
        consoles_list,
    };
    Template::render("consoles", context)
}

#[post("/add_console", data = "<form_data>")]
pub fn add_console(conn: HomeDbConn, form_data: Form<ConsoleFormData>) -> Redirect {
    use schema::consoles::dsl::consoles;

    let new_console = ConsoleTransport {
        _id: None,
        short_name: &form_data.short_name,
        long_name: &form_data.long_name,
    };
    let _result = diesel::insert_into(consoles)
        .values(&new_console)
        .execute(&*conn);

    Redirect::to("/consoles")
}

#[post("/update_console", data = "<form_data>")]
pub fn update_console(conn: HomeDbConn, form_data: Form<ConsoleFormData>) -> Redirect {
    use schema::consoles::dsl::{_id, consoles};

    let mod_console = ConsoleTransport {
        _id: form_data.id,
        short_name: &form_data.short_name,
        long_name: &form_data.long_name,
    };

    if let Some(mod_id) = mod_console._id {
        let _result = diesel::update(consoles.filter(_id.eq(mod_id)))
            .set(mod_console)
            .execute(&*conn);
    }
    Redirect::to("/consoles")
}

#[post("/delete_console", data = "<form_data>")]
pub fn delete_console(conn: HomeDbConn, form_data: Form<ConsoleFormData>) -> Redirect {
    use schema::consoles::dsl::{_id, consoles};

    if let Some(del_id) = form_data.id {
        let _result = diesel::delete(consoles.filter(_id.eq(del_id))).execute(&*conn);
    } else {
        println!("No id found to delete console: {:?}", *form_data);
    }

    Redirect::to("/consoles")
}
