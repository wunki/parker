use tide::{Body, Request, Response, StatusCode};

use crate::State;

#[derive(serde::Serialize)]
pub struct HealthCheckResponse {
    pub status: HealthCheckStatus,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum HealthCheckStatus {
    Pass,
}

pub async fn handle_health(_req: Request<State>) -> tide::Result {
    let check = HealthCheckResponse { status: HealthCheckStatus::Pass };
    Ok(Response::builder(StatusCode::Ok).body(Body::from_json(&check)?).build())
}
