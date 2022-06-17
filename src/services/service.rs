extern crate diesel;
use super::schema;
use super::models::{NovoUsuario, Usuario};
use super::lib;

use self::diesel::{prelude::*, insert_into};

pub fn get_users() -> Vec<Usuario> {
    use schema::usuario::dsl::*;

    let connection = lib::establish_connection();
    let results = usuario
        .load::<Usuario>(&connection)
        .expect("Falha ao recuperar usuário");
    results
}

pub fn post_user(new_user: NovoUsuario) -> Usuario {
    use schema::usuario::dsl::*;

    let connection = lib::establish_connection();
    insert_into(usuario)
        .values(&new_user)
        .execute(&connection)
        .expect("Erro ao inserir usuário");
    

    let result = usuario
        .order(id.desc())
        .first::<Usuario>(&connection)
        .expect("Falha ao recuperar usuário inserido");
    result
}
