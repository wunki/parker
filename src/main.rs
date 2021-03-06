use std::net::SocketAddr;

use parker::{app, settings::Settings};

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "parker=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    let settings = Settings::new().unwrap();
    let app = app();

    let addr = SocketAddr::from(([127, 0, 0, 1], settings.port));
    tracing::debug!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
