use chrono::Local;
use colored::*;
use std::env;

use crate::{
    config::Config,
    git,
};

fn segment(icon: &str, text: &str) -> String {
    format!(
        " {} {} ",
        icon.cyan(),
        text.white()
    )
}

pub fn render(config: &Config) -> String {

    let theme = config.theme();

    let mut prompt = String::new();


    prompt.push_str(
        &format!(
            "{}\n",
            theme.name.bright_red()
        )
    );


    if theme.show_git && config.git_enabled() {

        if let Some(branch) = git::branch() {

            prompt.push_str(
                &segment(
                    "",
                    &format!("git: {}", branch)
                )
            );

            prompt.push('\n');
        }
    }


    if theme.show_user {

        let user = whoami::username();

        let hostname =
            hostname::get()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();


        let identity =
            if theme.show_hostname {
                format!("{}@{}", user, hostname)
            } else {
                user
            };


        prompt.push_str(
            &segment(
                "◉",
                &identity
            )
        );

        prompt.push('\n');
    }


    if theme.show_directory {

        let directory =
            env::current_dir()
                .unwrap()
                .display()
                .to_string();


        prompt.push_str(
            &segment(
                "📁",
                &directory
            )
        );
    }


    if theme.show_time {

        let time =
            Local::now()
                .format("%H:%M")
                .to_string();


        prompt.push_str(
            &format!(
                " {}\n",
                time.yellow()
            )
        );
    }


    prompt.push_str(
        &format!(
            "{} ",
            theme.prompt_symbol.bright_red()
        )
    );


    prompt
}
