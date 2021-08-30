use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use tower::ServiceExt;

use parker::app;

#[tokio::test]
async fn welcome_returns_200_ok() {
    let app = app();
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
