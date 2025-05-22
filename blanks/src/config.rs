use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub language: String,
    pub diff: f64,
    pub problem: Option<usize>,
    pub mode: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language: "python".to_string(),
            diff: 0.2,
            problem: None,
            mode: "random".to_string(),
        }
    }
}

pub fn load_config() -> Config {
    let config_path = Path::new("blanks_config.json");
    
    if config_path.exists() {
        match fs::read_to_string(config_path) {
            Ok(contents) => {
                match serde_json::from_str(&contents) {
                    Ok(config) => return config,
                    Err(e) => {
                        eprintln!("Error parsing config file: {}", e);
                    }
                }
            },
            Err(e) => {
                eprintln!("Error reading config file: {}", e);
            }
        }
    }
    
    Config::default()
}

pub fn save_config(config: &Config) -> Result<(), std::io::Error> {
    let config_path = Path::new("blanks_config.json");
    let json = serde_json::to_string_pretty(config)?;
    fs::write(config_path, json)
}
