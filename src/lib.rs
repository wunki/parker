use axum::{handler::get, routing::BoxRoute, Router};
use handlers::{handle_health, handle_welcome};
use sqlx::{PgPool, Pool};

pub mod handlers;
pub mod settings;

#[derive(Clone, Debug)]
pub struct State {
    db_pool: PgPool,
}

/// Creates a database pool which will be re-used across the application.
pub async fn make_db_pool(db_url: &str) -> PgPool {
    Pool::connect(db_url)
        .await
        .expect("could not create database pool")
}

/// Returns the application containing all routes.
pub fn app() -> Router<BoxRoute> {
    let app = Router::new()
        .route("/", get(handle_welcome))
        .route("/health", get(handle_health))
        .boxed();
    app
}
