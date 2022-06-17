use super::schema::usuario;
use diesel::{Queryable, Insertable};
use rocket::serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Usuario {
    pub id: Option<i32>,
    pub nome: String,
    pub email: String,
    pub senha: String
}

#[derive(Insertable, Deserialize)] 
#[serde(crate = "rocket::serde")]
#[table_name = "usuario"]
pub struct NovoUsuario {
    pub nome: String,
    pub email: String,
    pub senha: String
}

