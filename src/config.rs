use std::fs;

use dirs::home_dir;
use serde::Deserialize;

use crate::theme::Theme;

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

impl Config {
    pub fn theme(&self) -> Theme {
        Theme::load(
            self.theme
                .as_deref()
                .unwrap_or("default")
        )
    }

    pub fn history_size(&self) -> usize {
        self.history_size.unwrap_or(5000)
    }

    pub fn git_enabled(&self) -> bool {
        self.git_prompt.unwrap_or(true)
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
