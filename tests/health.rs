use tide_testing::TideTestingExt;

use parker::settings::Settings;

#[async_std::test]
async fn health_returns_200_ok() {
    let settings = Settings::new().unwrap();
    let app = parker::app(&settings).await;
    let response = app.get("/health").await.unwrap();
    assert_eq!(response.status(), tide::StatusCode::Ok);
}
