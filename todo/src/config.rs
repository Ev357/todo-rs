use std::{env, net::Ipv4Addr, path::PathBuf};

use color_eyre::eyre::{Result, eyre};
use serde::Deserialize;
use tokio::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_api_url")]
    pub api_url: String,
}

impl Config {
    pub async fn new() -> Self {
        Self::load().await.unwrap_or_default()
    }

    async fn load() -> Result<Self> {
        let base_dir = match env::var("XDG_CONFIG_HOME") {
            Ok(dir) if !dir.is_empty() => PathBuf::from(dir),
            _ => {
                let home = env::var("HOME")
                    .map_err(|_| eyre!("Neither XDG_CONFIG_HOME nor HOME is set"))?;
                PathBuf::from(home).join(".config")
            }
        };

        let config_path = base_dir.join("todo").join("config.toml");

        let config_str = fs::read_to_string(config_path).await?;
        let config: Config = toml::from_str(&config_str)?;

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_url: default_api_url(),
        }
    }
}

fn default_api_url() -> String {
    format!("http://{}:7630", Ipv4Addr::LOCALHOST)
}
