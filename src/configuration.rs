use std::{fs, path::Path};
use serde::{Serialize, Deserialize};

use crate::ApplicationError;

/// Initializing app configuration
pub fn init() -> Result<AppConfig, ConfigError> {
    let config: AppConfig;
    
    let config_path = Path::new(".config");
    if config_path.exists() {
        let content = fs::read_to_string(config_path)?;
        config = toml::from_str(&content)?;
    }
    else {
        config = AppConfig::default();
        let content = toml::to_string(&config)?;
        fs::write(config_path, &content)?;
    }

    return Ok(config);
}

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub image: ImageConfig,
    pub job: JobConfig,
}

#[derive(Serialize, Deserialize)]
pub struct ImageConfig {
    pub category: String,
    pub resolution: String,
}

#[derive(Serialize, Deserialize)]
pub struct JobConfig {
    pub interval: u32,
}

impl Default for AppConfig {

    /// Initialize default app configuration
    fn default() -> Self {
        Self { 
            image: ImageConfig {
                category: String::from("landscapes"),
                resolution: String::from("1920x1080"),
            }, 
            job: JobConfig {
                interval: 600
            } 
        }
    }
}


/// Configuration errors
pub enum ConfigError {
    /// Configuration is invalid
    InvalidConfiguration(String),
    IoError(String),
}

impl From<ConfigError> for ApplicationError {
    fn from(error: ConfigError) -> Self {
        ApplicationError { message: match error {
            ConfigError::InvalidConfiguration(message) => message,
            ConfigError::IoError(message) => message,
        } }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value.to_string())
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(error: toml::de::Error) -> Self {
        Self::InvalidConfiguration(error.message().to_string())
    }
}

impl From<toml::ser::Error> for ConfigError {
    fn from(error: toml::ser::Error) -> Self {
        Self::InvalidConfiguration(error.to_string())
    }
}

impl From<String> for ConfigError {
    fn from(message: String) -> Self {
        Self::InvalidConfiguration(message)
    }
}