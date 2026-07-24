use std::fs;

use dirs::home_dir;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub theme: Option<String>,
    pub history_size: Option<usize>,
    pub git_prompt: Option<bool>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: Some("default".into()),
            history_size: Some(5000),
            git_prompt: Some(true),
        }
    }
}

pub fn load() -> Config {
    let mut path = home_dir().unwrap();

    path.push(".astrarc");

    if !path.exists() {
        return Config::default();
    }

    let content = fs::read_to_string(path)
        .unwrap_or_default();

    toml::from_str(&content)
        .unwrap_or_default()
}
