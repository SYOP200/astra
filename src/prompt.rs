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

fn safe_icon<'a>(config: &'a Config, icon: &'a str, fallback: &'a str) -> &'a str {
    if config.appearance.nerd_fonts {
        icon
    } else {
        fallback
    }
}

fn safe_prompt_symbol(theme: &crate::theme::Theme, config: &Config) -> String {
    if config.appearance.nerd_fonts || theme.prompt_symbol.is_ascii() {
        theme.prompt_symbol.clone()
    } else {
        ">".into()
    }
}

fn safe_separator(theme: &crate::theme::Theme, config: &Config) -> String {
    if config.appearance.nerd_fonts || theme.separator.is_ascii() {
        theme.separator.clone()
    } else {
        "|".into()
    }
}

pub fn render(config: &Config) -> String {
    let theme = config.theme();
    let accent = theme.accent_color();
    let primary = theme.primary_color();
    let secondary = theme.secondary_color();
    let directory_color = theme.directory_color();
    let git_color = theme.git_color();
    let time_color = theme.time_color();

    let separator = safe_separator(&theme, config);
    let prompt_symbol = safe_prompt_symbol(&theme, config);
    let mut segments: Vec<String> = Vec::new();

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
        segments.push(segment(
            safe_icon(config, "", "user"),
            &identity,
            primary,
            secondary,
        ));
    }

    if theme.show_git {
        if let Some(branch) = git::branch() {
            segments.push(segment(
                safe_icon(config, "", "git"),
                &branch,
                accent,
                git_color,
            ));
        }
    }

    if theme.show_directory {
        let directory = env::current_dir().unwrap_or_default().display().to_string();
        segments.push(segment(
            safe_icon(config, "", "dir"),
            &directory,
            primary,
            directory_color,
        ));
    }

    if theme.show_time {
        let time = Local::now().format("%H:%M").to_string();
        segments.push(segment(
            safe_icon(config, "", "time"),
            &time,
            secondary,
            time_color,
        ));
    }

    if segments.is_empty() {
        format!("{} ", styled(&prompt_symbol, accent))
    } else {
        let prompt_line = segments.join(&format!(" {} ", styled(&separator, secondary)));
        format!(
            "{} {} ",
            prompt_line.trim_end(),
            styled(&prompt_symbol, accent)
        )
    }
}
