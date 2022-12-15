//! config

use serde::Deserialize;

#[derive(Deserialize)]
pub struct WebConfig {
    pub addr: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub web: WebConfig,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut builder = config::Config::builder();
        builder = builder.add_source(config::Environment::default());
        builder.build()?.try_deserialize()
    }
}
