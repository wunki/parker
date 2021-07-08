use tide::Server;

pub mod handlers;
pub mod settings;

use crate::handlers::{handle_health, handle_welcome};

#[derive(Clone, Debug)]
pub struct State {}

pub fn app() -> Server<State> {
    let state = State {};
    let mut app = tide::with_state(state);
    app.at("/").get(handle_welcome);
    app.at("/health").get(handle_health);
    app
}
