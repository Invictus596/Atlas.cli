use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

const CONFIG_FILENAME: &str = ".atlas.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub ceo_phone: String,
    pub client_whatsapp: String,
    pub dev_email: String,
    pub twilio_sid: Option<String>,
    pub twilio_token: Option<String>,
    pub twilio_from: Option<String>,
}

impl Config {
    /// Return the path to the `.atlas.toml` file in the current working directory.
    fn config_path() -> PathBuf {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(CONFIG_FILENAME)
    }

    /// Load config from `.atlas.toml` in the current working directory.
    /// Returns `None` if the file doesn't exist or can't be parsed.
    pub fn load() -> io::Result<Option<Self>> {
        let path = Self::config_path();
        if !path.exists() {
            return Ok(None);
        }
        let content = fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&content).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to parse {}: {}", path.display(), e),
            )
        })?;
        Ok(Some(config))
    }

    /// Save the config to `.atlas.toml` in the current working directory.
    pub fn save(&self) -> io::Result<()> {
        let path = Self::config_path();
        let content = toml::to_string_pretty(self).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to serialize config: {}", e),
            )
        })?;
        fs::write(&path, content)?;
        Ok(())
    }
}

/// Convenience: check whether a config file exists in the current directory.
pub fn config_exists() -> bool {
    Config::config_path().exists()
}
