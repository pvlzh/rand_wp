use std::{fs, path::Path};
use serde::{Serialize, Deserialize};
use crate::ApplicationError;

/// Initializing app configuration
pub fn init() -> Result<AppConfig, ConfigurationError> {
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

/// Application configuration
#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub image: ImageConfig,
    pub job: JobConfig,
}

/// Background image configuration
#[derive(Serialize, Deserialize)]
pub struct ImageConfig {
    pub category: String,
    pub resolution: String,
}

/// Job scheduler configuration
#[derive(Serialize, Deserialize)]
pub struct JobConfig {
    pub interval_sec: u64,
}

/// Implementation of the default behavior for the type AppConfig
impl Default for AppConfig {
    /// Initialize default app configuration
    fn default() -> Self {
        Self { 
            image: ImageConfig {
                category: String::from("landscapes"),
                resolution: String::from("1920x1080"),
            }, 
            job: JobConfig {
                interval_sec: 600,
            } 
        }
    }
}


/// Configuration errors
pub enum ConfigurationError {
    /// Configuration is invalid
    InvalidConfiguration(String),
    IoError(String),
}

/// Convert ConfigurationError into ApplicationError
impl From<ConfigurationError> for ApplicationError {
    fn from(error: ConfigurationError) -> Self {
        let error = match error {
            ConfigurationError::InvalidConfiguration(message) => message,
            ConfigurationError::IoError(message) => message,
        };
        Self(error.to_string())
    }
}

/// Convert io Error into ConfigurationError
impl From<std::io::Error> for ConfigurationError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value.to_string())
    }
}

/// Convert deserialization Error into ConfigurationError
impl From<toml::de::Error> for ConfigurationError {
    fn from(error: toml::de::Error) -> Self {
        Self::InvalidConfiguration(error.message().to_string())
    }
}
/// Convert serialization Error into ConfigurationError
impl From<toml::ser::Error> for ConfigurationError {
    fn from(error: toml::ser::Error) -> Self {
        Self::InvalidConfiguration(error.to_string())
    }
}

/// Convert String into ConfigurationError
impl From<String> for ConfigurationError {
    fn from(message: String) -> Self {
        Self::InvalidConfiguration(message)
    }
}