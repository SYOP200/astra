use chrono::Local;
use colored::*;
use std::env;

use crate::{
    config::Config,
    git
};

fn segment(icon: &str, text: &str) -> String {
    format!(
        " {} {} ",
        icon.cyan(),
        text.white()
    )
}

pub fn render(config: &Config) -> String {

    let mut top = String::new();

    top.push_str(
        &format!(
            "{}\n",
            "╭─ Astra".bright_cyan()
        )
    );


    if config.git_prompt.unwrap_or(true) {

        if let Some(branch) = git::branch() {

            top.push_str(
                &segment(
                    "",
                    &format!("git: {}", branch)
                )
            );

            top.push('\n');
        }
    }


    let user = whoami::username();

    let hostname =
        hostname::get()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();


    top.push_str(
        &segment(
            "◉",
            &format!(
                "{}@{}",
                user,
                hostname
            )
        )
    );

    top.push('\n');


    let directory =
        env::current_dir()
        .unwrap()
        .display()
        .to_string();


    top.push_str(
        &segment(
            "📁",
            &directory
        )
    );


    let time =
        Local::now()
        .format("%H:%M")
        .to_string();


    top.push_str(
        &format!(
            " {}\n",
            time.yellow()
        )
    );


    top.push_str(
        &format!(
            "{} ",
            "╰─❯".bright_cyan()
        )
    );


    top
}
