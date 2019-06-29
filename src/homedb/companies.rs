use rocket;
use rocket::get;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib;
use rocket_contrib::databases::diesel::{AsChangeset, Insertable, Queryable};
use rocket_contrib::templates::Template;
use serde_derive::Serialize;

use super::HomeDbConn;
use crate::schema::game_companies;
use crate::*;

#[derive(Queryable, Serialize)]
pub struct Company {
    pub _id: i32,
    pub company_name_en: String,
    pub company_name_ja: Option<String>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "game_companies"]
pub struct CompanyTransport<'a> {
    pub _id: Option<i32>,
    pub company_name_en: &'a str,
    pub company_name_ja: Option<&'a str>,
}

#[derive(Serialize)]
pub struct CompaniesTemplateContext {
    err_msg: Option<String>,
    companies_list: Vec<Company>,
}

#[derive(FromForm, Debug)]
pub struct CompanyFormData {
    id: Option<i32>,
    company_name_en: String,
    company_name_ja: Option<String>,
}

// ----- BINDINGS -----

#[get("/companies")]
pub fn list_companies(conn: HomeDbConn) -> Template {
    use schema::game_companies::dsl::game_companies;

    let companies_list_result = game_companies.load::<Company>(&*conn);
    let (err_msg, companies_list) = match companies_list_result {
        Ok(list) => (None, list),
        Err(_e) => (Some("Unable to get companies list :(".to_owned()), vec![]),
    };

    let context = CompaniesTemplateContext {
        err_msg,
        companies_list,
    };
    Template::render("companies", context)
}

#[post("/add_company", data = "<form_data>")]
pub fn add_company(conn: HomeDbConn, form_data: Form<CompanyFormData>) -> Redirect {
    use schema::game_companies::dsl::game_companies;

    let new_company = CompanyTransport {
        _id: None,
        company_name_en: &form_data.company_name_en,
        company_name_ja: if let Some(ja) = &form_data.company_name_ja {
            Some(ja)
        } else {
            None
        },
    };
    let _result = diesel::insert_into(game_companies)
        .values(&new_company)
        .execute(&*conn);

    Redirect::to("/companies")
}

#[post("/update_company", data = "<form_data>")]
pub fn update_company(conn: HomeDbConn, form_data: Form<CompanyFormData>) -> Redirect {
    use schema::game_companies::dsl::{_id, game_companies};

    let mod_company = CompanyTransport {
        _id: form_data.id,
        company_name_en: &form_data.company_name_en,
        company_name_ja: if let Some(ja) = &form_data.company_name_ja {
            Some(ja)
        } else {
            None
        },
    };

    if let Some(mod_id) = mod_company._id {
        let _result = diesel::update(game_companies.filter(_id.eq(mod_id)))
            .set(mod_company)
            .execute(&*conn);
    }
    Redirect::to("/companies")
}

#[post("/delete_company", data = "<form_data>")]
pub fn delete_company(conn: HomeDbConn, form_data: Form<CompanyFormData>) -> Redirect {
    use schema::game_companies::dsl::{_id, game_companies};

    if let Some(del_id) = form_data.id {
        let _result = diesel::delete(game_companies.filter(_id.eq(del_id))).execute(&*conn);
    } else {
        println!("No id found to delete company: {:?}", *form_data);
    }

    Redirect::to("/companies")
}
