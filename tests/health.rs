use tide_testing::TideTestingExt;

#[async_std::test]
async fn health_returns_200_ok() {
    let app = parker::app();
    let response = app.get("/health").await.unwrap();
    assert_eq!(response.status(), tide::StatusCode::Ok);
}
