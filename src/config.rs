{% if include-config %}
use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub app_name: String,
    pub debug: bool,
    // Add more configuration fields as needed
}

impl AppConfig {
    pub fn new(config_path: Option<&str>) -> Result<Self, ConfigError> {
        let mut config_builder = Config::builder();
        
        // Add default config
        config_builder = config_builder.add_source(File::from_str(
            r#"
            {
                "app_name": "{{cli-name}}",
                "debug": false
            }
            "#,
            config::FileFormat::Json,
        ));
        
        // Add user config if provided
        if let Some(path) = config_path {
            if Path::new(path).exists() {
                config_builder = config_builder.add_source(File::with_name(path));
            }
        }
        
        // Build and deserialize
        let config = config_builder.build()?;
        config.try_deserialize()
    }
}
{% endif %}