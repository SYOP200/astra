use serde::Deserialize;
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
};

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
    pub show_user: bool,
    pub show_hostname: bool,
    pub show_directory: bool,
    pub show_git: bool,
    pub show_time: bool,
    pub symbol: String,
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
            theme: "crimson".into(),
            startup_message: true,
            update_check: false,
            telemetry: false,
        }
    }
}


impl Default for PromptConfig {
    fn default() -> Self {
        Self {
            show_user: true,
            show_hostname: true,
            show_directory: true,
            show_git: true,
            show_time: true,
            symbol: "❯".into(),
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
            nerd_fonts: true,
            animations: true,
            compact: false,
        }
    }
}


impl Config {

    pub fn load() -> Self {
        let path =
            dirs::home_dir()
                .unwrap_or(PathBuf::from("."))
                .join(".astrarc");


        if !path.exists() {
            return Self::default();
        }


        let contents =
            fs::read_to_string(path)
                .unwrap_or_default();


        toml::from_str(&contents)
            .unwrap_or_default()
    }


    pub fn alias(&self, name: &str) -> Option<&String> {
        self.aliases.get(name)
    }
}


pub fn load() -> Config {
    Config::load()
}
