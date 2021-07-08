use tide::{Body, Request, Response, StatusCode};

pub async fn handle_welcome(_req: Request<()>) -> tide::Result {
    Ok(Response::builder(StatusCode::Ok)
        .body(Body::from_string(String::from("Welcome!")))
        .build())
}
