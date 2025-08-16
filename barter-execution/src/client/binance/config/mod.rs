use std::{env, fs, path::Path};

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
struct NetworkConfigFile {
    api: NetworkBaseUrls,
    credentials: Credentials,
}

#[derive(Debug, Clone, Deserialize)]
struct NetworkBaseUrls {
    http: String,
    websocket: String,
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
        let general = read_toml::<GeneralConfig>(config_dir.join("general.toml"))?;
        let network_file = if general.api.testnet { "testnet.toml" } else { "mainnet.toml" };
        let network = read_toml::<NetworkConfigFile>(config_dir.join(network_file))?;

        Ok(BinanceSpotConfig {
            http_base_url: network.api.http,
            websocket_base_url: network.api.websocket,
            api_key: expand_env_var(&network.credentials.api_key)?,
            api_secret: expand_env_var(&network.credentials.api_secret)?,
            is_testnet: general.api.testnet,
        })
    }
}

fn read_toml<T: for<'de> Deserialize<'de>>(path: impl AsRef<Path>) -> Result<T, ConfigError> {
    let path = path.as_ref();
    let content = fs::read_to_string(path)
        .map_err(|e| ConfigError::FileRead(path.display().to_string(), e))?;

    toml::from_str(&content)
        .map_err(|e| ConfigError::Parse(path.display().to_string(), e))
}

fn expand_env_var(value: &str) -> Result<String, ConfigError> {
    if value.starts_with("${") && value.ends_with('}') {
        let var_name = &value[2..value.len() - 1];
        env::var(var_name).map_err(|_| ConfigError::EnvVar(var_name.to_string()))
    } else {
        Ok(value.to_string())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read '{0}': {1}")]
    FileRead(String, std::io::Error),

    #[error("Failed to parse '{0}': {1}")]
    Parse(String, toml::de::Error),

    #[error("Environment variable '{0}' not found")]
    EnvVar(String),
}