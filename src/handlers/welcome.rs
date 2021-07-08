use tide::{Body, Request, Response, StatusCode};

use crate::State;

pub async fn handle_welcome(_req: Request<State>) -> tide::Result {
    Ok(Response::builder(StatusCode::Ok)
        .body(Body::from_string(String::from("Welcome!")))
        .build())
}
