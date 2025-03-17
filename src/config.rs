{% if use_config %}
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::error::{Result, Error};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub default_greeting: String,
    pub processing: ProcessingConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessingConfig {
    pub default_mode: String,
    pub max_file_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_greeting: "Hello".to_string(),
            processing: ProcessingConfig {
                default_mode: "normal".to_string(),
                max_file_size: 10_485_760, // 10 MB
            },
        }
    }
}

impl Config {
    pub fn load(path: Option<String>) -> Result<Self> {
        match path {
            Some(config_path) => {
                let path = Path::new(&config_path);
                if !path.exists() {
                    return Err(Error::Config(format!("Config file not found: {}", config_path)));
                }
                
                let content = fs::read_to_string(path)?;
                let config: Config = toml::from_str(&content)
                    .map_err(|e| Error::Config(format!("Failed to parse config: {}", e)))?;
                
                Ok(config)
            },
            None => {
                // Try to load from default locations
                let default_paths = [
                    "config.toml",
                    "{{cli_name}}.toml",
                ];
                
                for path in default_paths {
                    if Path::new(path).exists() {
                        let content = fs::read_to_string(path)?;
                        if let Ok(config) = toml::from_str(&content) {
                            return Ok(config);
                        }
                    }
                }
                
                // No config found, use defaults
                Ok(Config::default())
            }
        }
    }
}{% endif %}