extern crate user_api;
extern crate diesel;

use crate::domain::models::Usuario;

use self::user_api::*;
use self::diesel::prelude::*;

pub fn get_users() -> Vec<Usuario> {
    use user_api::schema::usuario::dsl::*;

    let connection = establish_connection();
    let results = usuario
        .load::<Usuario>(&connection)
        .expect("Falha ao recuperar usuário");
    results
}