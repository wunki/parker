#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    pub status: HealthCheckStatus 
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum HealthCheckStatus {
    Pass
}

/// This is where we explain how to make use of our
/// service. Best is to guide the user towards their
/// first step.
#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

/// This is where we will indicate if our service
/// is healthy. Check connection to external systems,
/// like the database.
#[get("/health")]
pub fn health_check() -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse { status: HealthCheckStatus::Pass })
}

/// Generates the rocket server and launches it when needed.
#[launch]
pub fn rocket() -> _ {
    rocket::build().mount("/", routes![index, health_check])
}
