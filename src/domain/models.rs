use diesel::Queryable;
use rocket::serde::Serialize;

#[derive(Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Usuario {
    pub id: Option<i32>,
    pub nome: String,
    pub email: String,
    pub senha: String
}