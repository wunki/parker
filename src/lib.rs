use sqlx::{PgPool, Pool};
use tide::Server;

pub mod handlers;
pub mod settings;

use crate::handlers::{handle_health, handle_welcome};
use crate::settings::Settings;

#[derive(Clone, Debug)]
pub struct State {
    db_pool: PgPool,
}

pub async fn make_db_pool(db_url: &str) -> PgPool {
    Pool::connect(db_url).await.expect("could not create database pool")
}

pub async fn app(settings: &Settings) -> Server<State> {
    let state = State {
        db_pool: make_db_pool(&settings.database_url).await,
    };
    let mut app = tide::with_state(state);
    app.at("/").get(handle_welcome);
    app.at("/health").get(handle_health);
    app
}
