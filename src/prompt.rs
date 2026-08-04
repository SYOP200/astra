use chrono::Local;
use crossterm::style::{Color, Stylize};
use std::env;

use crate::{config::Config, git};

fn styled(text: &str, color: Color) -> String {
    format!("{}", text.with(color))
}

fn segment(icon: &str, text: &str, icon_color: Color, text_color: Color) -> String {
    format!(
        " {} {} ",
        styled(icon, icon_color),
        styled(text, text_color)
    )
}

pub fn render(config: &Config) -> String {
    let theme = config.theme();
    let accent = theme.accent_color();
    let primary = theme.primary_color();
    let secondary = theme.secondary_color();
    let directory_color = theme.directory_color();
    let git_color = theme.git_color();
    let time_color = theme.time_color();

    let mut prompt = String::new();
    let layout = theme.layout.to_ascii_lowercase();

    if layout == "minimal" {
        if theme.show_directory {
            let directory = env::current_dir().unwrap_or_default().display().to_string();
            prompt.push_str(&styled(&format!("📁 {} ", directory), directory_color));
        }

        if theme.show_git {
            if let Some(branch) = git::branch() {
                prompt.push_str(&styled(&format!(" {} ", branch), git_color));
            }
        }

        prompt.push_str(&format!("{} ", styled(&theme.prompt_symbol, accent)));
        return prompt;
    }

    prompt.push_str(&styled(&theme.name, accent));
    prompt.push('\n');

    if theme.show_git {
        if let Some(branch) = git::branch() {
            prompt.push_str(&segment(
                "",
                &format!("git: {}", branch),
                accent,
                git_color,
            ));
            prompt.push('\n');
        }
    }

    if theme.show_user {
        let user = whoami::username();
        let hostname = hostname::get()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let identity = if theme.show_hostname {
            format!("{}@{}", user, hostname)
        } else {
            user
        };

        prompt.push_str(&segment("◉", &identity, primary, secondary));
        prompt.push('\n');
    }

    if theme.show_directory {
        let directory = env::current_dir().unwrap_or_default().display().to_string();
        prompt.push_str(&segment("📁", &directory, primary, directory_color));
    }

    if theme.show_time {
        let time = Local::now().format("%H:%M").to_string();
        prompt.push_str(&format!("{}\n", styled(&time, time_color)));
    }

    prompt.push_str(&format!("{} ", styled(&theme.prompt_symbol, accent)));
    prompt
}
