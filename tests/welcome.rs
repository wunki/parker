use tide_testing::TideTestingExt;

#[async_std::test]
async fn welcome_returns_200_ok() {
    let app = parker::app();
    let response = app.get("/").await.unwrap();
    assert_eq!(response.status(), tide::StatusCode::Ok);
}
