use serde::Deserialize;
use std::{collections::HashMap, fs, path::PathBuf};
use toml::Value;

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Config {
    pub general: GeneralConfig,
    pub prompt: PromptConfig,
    pub history: HistoryConfig,
    pub completion: CompletionConfig,
    pub behavior: BehaviorConfig,
    pub aliases: HashMap<String, String>,
    pub plugins: PluginConfig,
    pub appearance: AppearanceConfig,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct GeneralConfig {
    pub theme: String,
    pub startup_message: bool,
    pub update_check: bool,
    pub telemetry: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct PromptConfig {
    pub show_user: Option<bool>,
    pub show_hostname: Option<bool>,
    pub show_directory: Option<bool>,
    pub show_git: Option<bool>,
    pub show_time: Option<bool>,
    pub symbol: Option<String>,
    pub separator: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct HistoryConfig {
    pub enabled: bool,
    pub file: String,
    pub size: usize,
    pub deduplicate: bool,
    pub ignore_duplicates: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct CompletionConfig {
    pub enabled: bool,
    pub case_sensitive: bool,
    pub show_hidden_files: bool,
    pub max_results: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct BehaviorConfig {
    pub confirm_exit: bool,
    pub allow_scripts: bool,
    pub auto_cd: bool,
    pub vi_mode: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct PluginConfig {
    pub enabled: bool,
    pub directory: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct AppearanceConfig {
    pub unicode: bool,
    pub nerd_fonts: bool,
    pub animations: bool,
    pub compact: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            prompt: PromptConfig::default(),
            history: HistoryConfig::default(),
            completion: CompletionConfig::default(),
            behavior: BehaviorConfig::default(),
            aliases: HashMap::new(),
            plugins: PluginConfig::default(),
            appearance: AppearanceConfig::default(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            theme: "default".into(),
            startup_message: true,
            update_check: false,
            telemetry: false,
        }
    }
}

impl Default for PromptConfig {
    fn default() -> Self {
        Self {
            show_user: None,
            show_hostname: None,
            show_directory: None,
            show_git: None,
            show_time: None,
            symbol: None,
            separator: None,
        }
    }
}

impl Default for HistoryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            file: "~/.astra_history".into(),
            size: 10000,
            deduplicate: true,
            ignore_duplicates: true,
        }
    }
}

impl Default for CompletionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            case_sensitive: false,
            show_hidden_files: false,
            max_results: 20,
        }
    }
}

impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            confirm_exit: true,
            allow_scripts: true,
            auto_cd: false,
            vi_mode: false,
        }
    }
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            directory: "~/.astra/plugins".into(),
        }
    }
}

impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {
            unicode: true,
            nerd_fonts: false,
            animations: true,
            compact: false,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let path = dirs::home_dir()
            .unwrap_or(PathBuf::from("."))
            .join(".astrarc");

        if !path.exists() {
            return Self::default();
        }

        let contents = fs::read_to_string(path).unwrap_or_default();

        toml::from_str(&contents).unwrap_or_default()
    }

    pub fn theme(&self) -> crate::theme::Theme {
        let mut theme = crate::theme::load(&self.general.theme);

        if let Some(show_user) = self.prompt.show_user {
            theme.show_user = show_user;
        }
        if let Some(show_hostname) = self.prompt.show_hostname {
            theme.show_hostname = show_hostname;
        }
        if let Some(show_directory) = self.prompt.show_directory {
            theme.show_directory = show_directory;
        }
        if let Some(show_git) = self.prompt.show_git {
            theme.show_git = show_git;
        }
        if let Some(show_time) = self.prompt.show_time {
            theme.show_time = show_time;
        }
        if let Some(symbol) = &self.prompt.symbol {
            theme.prompt_symbol = symbol.clone();
        }
        if let Some(separator) = &self.prompt.separator {
            theme.separator = separator.clone();
        }

        theme
    }

    pub fn save_theme(theme_name: &str) -> Result<(), std::io::Error> {
        let path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".astrarc");
        let mut document = if path.exists() {
            let contents = fs::read_to_string(&path).unwrap_or_default();
            toml::from_str(&contents).unwrap_or_else(|_| Value::Table(toml::map::Map::new()))
        } else {
            Value::Table(toml::map::Map::new())
        };

        let table = document.as_table_mut().unwrap();
        let general = table
            .entry("general")
            .or_insert_with(|| Value::Table(toml::map::Map::new()));

        if let Value::Table(general) = general {
            general.insert("theme".into(), Value::String(theme_name.into()));
        }

        let contents = toml::to_string_pretty(&document).unwrap_or_default();
        fs::write(path, contents)
    }
}

pub fn load() -> Config {
    Config::load()
}
