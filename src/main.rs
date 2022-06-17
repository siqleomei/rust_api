#[macro_use]
extern crate diesel;

#[macro_use] 
extern crate rocket;
use rocket::serde::json::Json;

pub mod domain;
pub mod services;
pub mod schema;

use domain::models::Usuario;
use services::service::{get_users};

#[get("/users")]
fn index() -> Json<Vec<Usuario>> {
    Json(get_users())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}