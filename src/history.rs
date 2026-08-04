use std::path::PathBuf;

use rustyline::{history::DefaultHistory, Editor};

use crate::{completion::AstraCompleter, config::Config};

fn expand_home(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(rest);
        }
    }

    PathBuf::from(path)
}

pub fn load(readline: &mut Editor<AstraCompleter, DefaultHistory>, config: &Config) {
    if !config.history.enabled {
        return;
    }

    let path = expand_home(&config.history.file);

    let _ = readline.load_history(&path);
}

pub fn save(readline: &mut Editor<AstraCompleter, DefaultHistory>, config: &Config) {
    if !config.history.enabled {
        return;
    }

    let path = expand_home(&config.history.file);

    let _ = readline.save_history(&path);
}
