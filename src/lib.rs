pub mod configuration;
pub mod routes;

#[macro_use]
extern crate rocket;

use crate::routes::{get_welcome, get_health_check};

/// Generates the rocket server and launches it when needed.
#[launch]
pub fn rocket() -> _ {
    rocket::build().mount("/", routes![get_welcome, get_health_check])
}
