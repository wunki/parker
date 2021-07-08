use tide::Server;

pub mod handlers;
pub mod settings;

use crate::handlers::{handle_health, handle_welcome};
use crate::settings::Settings;

pub fn app() -> Server<()> {
    let mut app = tide::new();
    app.at("/").get(handle_welcome);
    app.at("/health").get(handle_health);
    app
}
