use rocket;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib;
use rocket_contrib::databases::diesel::{AsChangeset, Insertable, Queryable};
use rocket_contrib::templates::Template;
use serde_derive::Serialize;

use super::HomeDbConn;
use crate::schema::genres;
use crate::*;

#[derive(Queryable, Serialize)]
pub struct Genre {
    pub _id: i32,
    pub genre: String,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "genres"]
pub struct GenreTransport<'a> {
    pub _id: Option<i32>,
    pub genre: &'a str,
}

#[derive(Serialize)]
pub struct GenresTemplateContext {
    err_msg: Option<String>,
    genres_list: Vec<Genre>,
}

#[derive(FromForm, Debug)]
pub struct GenreFormData {
    id: Option<i32>,
    genre: String,
}

// ----- BINDINGS -----

#[get("/genres")]
pub fn list_genres(conn: HomeDbConn) -> Template {
    use schema::genres::dsl::genres;

    let genres_list_result = genres.load::<Genre>(&*conn);
    let (err_msg, genres_list) = match genres_list_result {
        Ok(list) => (None, list),
        Err(e) => (Some("Unable to get genres list :(".to_owned()), vec![]),
    };

    let context = GenresTemplateContext {
        err_msg,
        genres_list,
    };
    Template::render("genres", context)
}

#[post("/add_genre", data = "<form_data>")]
pub fn add_genre(conn: HomeDbConn, form_data: Form<GenreFormData>) -> Redirect {
    use schema::genres::dsl::genres;

    let new_genre = GenreTransport {
        _id: None,
        genre: &form_data.genre,
    };
    let _result = diesel::insert_into(genres)
        .values(&new_genre)
        .execute(&*conn);

    Redirect::to("/genres")
}

#[post("/update_genre", data = "<form_data>")]
pub fn update_genre(conn: HomeDbConn, form_data: Form<GenreFormData>) -> Redirect {
    use schema::genres::dsl::{_id, genres};

    let mod_genre = GenreTransport {
        _id: form_data.id,
        genre: &form_data.genre,
    };

    if let Some(mod_id) = mod_genre._id {
        let _result = diesel::update(genres.filter(_id.eq(mod_id)))
            .set(mod_genre)
            .execute(&*conn);
    }
    Redirect::to("/genres")
}

#[post("/delete_genre", data = "<form_data>")]
pub fn delete_genre(conn: HomeDbConn, form_data: Form<GenreFormData>) -> Redirect {
    use schema::genres::dsl::{_id, genres};

    if let Some(del_id) = form_data.id {
        let _result = diesel::delete(genres.filter(_id.eq(del_id))).execute(&*conn);
    } else {
        println!("No id found to delete genre: {:?}", *form_data);
    }

    Redirect::to("/genres")
}
