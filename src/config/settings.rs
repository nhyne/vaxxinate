use config::{Config, ConfigError, Environment, File};
use std::env;

#[derive(Debug, Deserialize)]
pub struct Window {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Deserialize)]
pub struct Player {
    pub spawn_point: Point,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub window: Window,
    pub player: Player,
}

impl Settings {
    // From: https://github.com/mehcode/config-rs/blob/0.9.3/examples/hierarchical-env/src/settings.rs
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("config/default"))?;

        let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        s.merge(File::with_name("config/local").required(false))?;

        s.merge(Environment::with_prefix("ZOMBIES_"))?;

        s.try_into()
    }
}
