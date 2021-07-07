use rocket::serde::json::Json;

#[derive(serde::Serialize)]
pub struct HealthCheckResponse {
    pub status: HealthCheckStatus,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum HealthCheckStatus {
    Pass,
}

/// This is where we will indicate if our service
/// is healthy. Check connection to external systems,
/// like the database.
#[get("/health")]
pub fn get_health_check() -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse {
        status: HealthCheckStatus::Pass,
    })
}
