#[macro_use]
extern crate diesel;

#[macro_use] 
extern crate rocket;
use rocket::serde::json::Json;

pub mod domain;
pub mod services;
pub mod schema;
pub mod lib;

use domain::models::{Usuario, NovoUsuario};
use services::service::{get_users, post_user};

#[get("/")]
fn index() -> Json<Vec<Usuario>> {
    Json(get_users())
}

#[post("/", data = "<user>")]
fn insert(user: Json<NovoUsuario>) -> Json<Usuario> {
    Json(post_user(user.0))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/users", routes![index, insert])
}