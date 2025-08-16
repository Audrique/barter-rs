use std::{env, fs, path::Path};

use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
struct GeneralConfig {
    api: GeneralApiConfig,
}

#[derive(Debug, Clone, Deserialize)]
struct GeneralApiConfig {
    testnet: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct NetworkConfig {
    api: NetworkBaseUrls,
    credentials: Credentials,
}

#[derive(Debug, Clone, Deserialize)]
struct NetworkBaseUrls {
    http_base_url: String,
    websocket_base_url: String,
}

#[derive(Debug, Clone, Deserialize)]
struct Credentials {
    api_key: String,
    api_secret: String,
}

#[derive(Debug, Clone)]
pub struct BinanceSpotConfig {
    pub http_base_url: String,
    pub websocket_base_url: String,
    pub api_key: String,
    pub api_secret: String,
    pub is_testnet: bool,
}

impl BinanceSpotConfig {
    pub fn load() -> Result<Self, ConfigError> {
        Self::load_from_dir("config")
    }

    pub fn load_from_dir<P: AsRef<Path>>(config_dir: P) -> Result<Self, ConfigError> {
        let config_dir = config_dir.as_ref();
        let general_config = Config::builder()
            .add_source(File::from(config_dir.join("general.toml")))
            .build()?;

        let general: GeneralConfig = general_config.try_deserialize()?;
        let network_file = if general.api.testnet { "testnet.toml" } else { "mainnet.toml" };
        dotenvy::dotenv().expect("Failed to read .env file");
        let settings = Config::builder()
            .add_source(File::from(config_dir.join(network_file)))
            .add_source(Environment::default().separator("__"))
            .build()?;

        let network_config: NetworkConfig = settings.try_deserialize()?;
        Ok(BinanceSpotConfig {
            http_base_url: network_config.api.http_base_url,
            websocket_base_url: network_config.api.websocket_base_url,
            api_key: network_config.credentials.api_key,
            api_secret: network_config.credentials.api_secret,
            is_testnet: general.api.testnet,
        })
    }
}
