use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub host: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("configuration"))
        .build()
        .expect("Config file should be in root");

    settings.try_deserialize()
}
