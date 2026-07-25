use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Theme {
    pub name: String,
    pub prompt_symbol: String,
    pub show_user: bool,
    pub show_hostname: bool,
    pub show_directory: bool,
    pub show_git: bool,
    pub show_time: bool,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "Astra Dark".into(),
            prompt_symbol: "❯".into(),
            show_user: true,
            show_hostname: true,
            show_directory: true,
            show_git: true,
            show_time: true,
        }
    }
}

impl Theme {
    pub fn load(name: &str) -> Self {
        let path = format!("themes/{}.toml", name);

        if let Ok(data) = fs::read_to_string(path) {
            if let Ok(theme) = toml::from_str(&data) {
                return theme;
            }
        }

        Self::default()
    }
}
