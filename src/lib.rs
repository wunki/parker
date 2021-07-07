#[macro_use]
extern crate rocket;

pub mod routes;
pub mod settings;

use crate::routes::{get_health_check, get_welcome};
use crate::settings::Settings;

/// Generates the rocket server and launches it when needed.
#[launch]
pub fn rocket() -> _ {
    let settings = Settings::new();
    rocket::build().mount("/", routes![get_welcome, get_health_check])
}
