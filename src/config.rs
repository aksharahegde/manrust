use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub history: Vec<String>,
    pub favorites: Vec<String>,
    pub last_command: Option<String>,
    pub last_section: Option<u8>,
    pub theme: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .with_context(|| format!("Failed to read config file: {:?}", config_path))?;
            let config: Config = toml::from_str(&content)
                .with_context(|| "Failed to parse config file")?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {:?}", parent))?;
        }

        let content = toml::to_string_pretty(self)
            .with_context(|| "Failed to serialize config")?;
        fs::write(&config_path, content)
            .with_context(|| format!("Failed to write config file: {:?}", config_path))?;
        
        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Failed to find config directory")?;
        Ok(config_dir.join("man-tui").join("config.toml"))
    }

    pub fn add_to_history(&mut self, cmd: String) {
        self.history.retain(|x| x != &cmd);
        self.history.insert(0, cmd);
        if self.history.len() > 100 {
            self.history.truncate(100);
        }
    }

    pub fn toggle_favorite(&mut self, cmd: String) -> bool {
        if let Some(pos) = self.favorites.iter().position(|x| x == &cmd) {
            self.favorites.remove(pos);
            false
        } else {
            self.favorites.push(cmd);
            true
        }
    }

    pub fn is_favorite(&self, cmd: &str) -> bool {
        self.favorites.contains(&cmd.to_string())
    }
}

