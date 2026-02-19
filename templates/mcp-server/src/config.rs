use serde::Deserialize;
use std::path::PathBuf;
use tracing::{debug, warn};

#[derive(Debug, Deserialize, Default, Clone)]
pub struct AppConfig {
    // Add project-specific config fields here
}

impl AppConfig {
    pub fn load() -> Self {
        let config_path = Self::config_path();
        debug!(path = %config_path.display(), "Loading config");

        if config_path.exists() {
            match std::fs::read_to_string(&config_path) {
                Ok(contents) => match toml::from_str::<AppConfig>(&contents) {
                    Ok(config) => {
                        debug!("Config loaded successfully");
                        return config;
                    }
                    Err(e) => {
                        warn!(error = %e, "Failed to parse config file, using defaults");
                    }
                },
                Err(e) => {
                    warn!(error = %e, "Failed to read config file, using defaults");
                }
            }
        } else {
            debug!("Config file not found, using defaults");
        }

        Self::default()
    }

    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("{{PROJECT_NAME}}")
            .join("config.toml")
    }
}
