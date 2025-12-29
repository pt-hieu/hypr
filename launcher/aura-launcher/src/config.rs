use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub launcher: LauncherConfig,
    #[serde(default)]
    pub appearance: AppearanceConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LauncherConfig {
    #[serde(default = "default_max_results")]
    pub max_results: usize,
    #[serde(default = "default_width")]
    pub width: i32,
    #[serde(default = "default_height")]
    pub height: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppearanceConfig {
    #[serde(default)]
    pub icon_theme: Option<String>,
    #[serde(default = "default_icon_size")]
    pub icon_size: u16,
}

fn default_max_results() -> usize { 10 }
fn default_width() -> i32 { 600 }
fn default_height() -> i32 { 400 }
fn default_icon_size() -> u16 { 48 }

impl Default for LauncherConfig {
    fn default() -> Self {
        Self {
            max_results: default_max_results(),
            width: default_width(),
            height: default_height(),
        }
    }
}

impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {
            icon_theme: None,
            icon_size: default_icon_size(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            launcher: LauncherConfig::default(),
            appearance: AppearanceConfig::default(),
        }
    }
}

impl Config {
    /// Load config from XDG config directory
    pub fn load() -> Self {
        let path = Self::config_path();

        if let Ok(contents) = fs::read_to_string(&path) {
            if let Ok(config) = toml::from_str(&contents) {
                return config;
            }
        }

        Config::default()
    }

    /// Get config file path
    fn config_path() -> PathBuf {
        let xdg = xdg::BaseDirectories::with_prefix("aura-launcher")
            .expect("Failed to get XDG directories");

        xdg.get_config_file("config.toml")
    }

    /// Get the CSS file path
    pub fn css_path() -> Option<PathBuf> {
        let xdg = xdg::BaseDirectories::with_prefix("aura-launcher").ok()?;
        let path = xdg.get_config_file("style.css");
        if path.exists() {
            Some(path)
        } else {
            None
        }
    }
}
