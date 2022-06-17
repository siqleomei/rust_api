#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;

use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
            .expect("Falha ao recuperar variavel de ambiente DATABASE_URL");
    SqliteConnection::establish(&database_url)
            .expect(&format!("Erro ao realizar conexão com {}", database_url))
}