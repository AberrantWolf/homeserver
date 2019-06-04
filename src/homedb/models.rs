use rocket_contrib::databases::diesel::Queryable;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Queryable)]
pub struct Log {
    pub id: i32,
    pub msg: String,
}