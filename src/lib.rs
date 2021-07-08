use tide::Server;

pub mod routes;
pub mod settings;

use crate::routes::{handle_health, handle_welcome};
use crate::settings::Settings;

pub fn app() -> Server<()> {
    let _settings = Settings::new();
    let mut app = tide::new();
    app.at("/").get(handle_welcome);
    app.at("/health").get(handle_health);
    app
}
