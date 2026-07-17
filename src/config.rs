use std::env;
use std::num::ParseIntError;

pub struct Config {
    pub host: String,
    pub port: u16,
}

pub struct FbConfig {
    pub host: String,
    pub port: u16,
    pub file: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to parse integer from environment variable: {0}")]
    ParseInt(#[from] ParseIntError),
    // #[error("Missing environment variable: {0}")]
    // MissingVar(String),
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            host: env::var("HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
        })
    }
}

impl FbConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            host: env::var("FB_HOST")
                .unwrap_or_else(|_| "localhost".to_string()),
            port: env::var("FB_PORT")
                .unwrap_or_else(|_| "3050".to_string())
                .parse()?,
            file: env::var("FB_FILE")
                .unwrap_or_else(|_| "ShopManager".to_string()),
            user: env::var("FB_USER")
                .unwrap_or_else(|_| "SYSDBA".to_string()),
            password: env::var("FB_PASS")
                .unwrap_or_else(|_| "masterkey".to_string()),
        })
    }
}
