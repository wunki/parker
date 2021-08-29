use axum::{http::StatusCode, response::IntoResponse, response::Json};

#[derive(serde::Serialize)]
pub struct HealthCheckResponse {
    pub status: HealthCheckStatus,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum HealthCheckStatus {
    Pass,
}

pub async fn handle_health() -> impl IntoResponse {
    let check = HealthCheckResponse {
        status: HealthCheckStatus::Pass,
    };

    (StatusCode::OK, Json(check))
}
