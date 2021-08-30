use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use parker::app;
use tower::ServiceExt;

#[tokio::test]
async fn health_returns_200_ok() {
    let app = app();
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
