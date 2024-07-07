use std::path::Path;
use serde::{Serialize, Deserialize};
use tokio::fs;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
/// Errors of module configuration
pub enum Error {
    /// Configuration is invalid
    ReadConfigurationError(toml::de::Error),
    SaveConfigurationError(toml::ser::Error),
    IoError(std::io::Error),
}

impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Self {
        Self::ReadConfigurationError(error)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(error: toml::ser::Error) -> Self {
        Self::SaveConfigurationError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

/// Initializing app configuration
pub async fn init() -> Result<AppConfig> {
    let config: AppConfig;
    
    let config_path = Path::new(".config");
    if config_path.exists() {
        let content = fs::read_to_string(config_path).await?;
        config = toml::from_str(&content)?;
    }
    else {
        config = AppConfig::default();
        let content = toml::to_string(&config)?;
        fs::write(config_path, &content).await?;
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