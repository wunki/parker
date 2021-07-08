use config::{Config, ConfigError, Environment};

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub port: u16,
    pub database_url: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();
        s.merge(Environment::new())?;
        s.try_into()
    }
}
