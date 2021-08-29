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
