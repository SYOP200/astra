mod ast;
mod executor;
mod lexer;
mod parser;
mod resolver;

mod builtins;
mod completion;
mod config;
mod git;
mod history;
mod prompt;
mod shell;
mod theme;

use std::{env, path::Path};

fn print_usage() {
    println!("Astra Shell");
    println!();
    println!("Usage: astra [OPTIONS] [script.astra]");
    println!();
    println!("Options:");
    println!("  -h, --help           Show this help message");
    println!("  -V, --version        Print release version");
    println!("  --theme-list         List available themes");
    println!("  --theme-show         Show the current theme");
    println!("  --theme-set NAME     Set and save a theme name");
    println!("  --eval COMMAND       Evaluate a single command and exit");
    println!("  --config-example     Print a sample ~/.astrarc configuration file");
    println!("  [script.astra]       Execute an Astra script file");
    println!();
    println!("Run without arguments to start the Astra interactive shell.");
}

fn print_version() {
    println!("Astra {}", env!("CARGO_PKG_VERSION"));
}

fn print_config_example() {
    println!(
        r#"[general]
theme = \"crimson\"

[prompt]
show_user = true
show_hostname = true
show_directory = true
show_git = true
show_time = true
symbol = \"❯\"
"#
    );
}

fn run_theme_list() {
    println!("Available themes:");
    for theme_name in theme::available() {
        println!(" - {}", theme_name);
    }
}

fn run_theme_show() {
    let config = config::load();
    println!("Current theme: {}", config.general.theme);
}

fn run_theme_set(theme_name: &str) {
    let valid = theme::available()
        .iter()
        .any(|name| name.eq_ignore_ascii_case(theme_name));
    if !valid {
        eprintln!(
            "Unknown theme '{}'. Use --theme-list to see available themes.",
            theme_name
        );
        std::process::exit(1);
    }

    let canonical = theme_name.to_ascii_lowercase();
    if let Err(err) = config::Config::save_theme(&canonical) {
        eprintln!("Failed to save theme: {}", err);
        std::process::exit(1);
    }

    println!("Theme updated to '{}' in ~/.astrarc", canonical);
}

fn run_script(path: &Path) {
    let config = config::load();
    match shell::run_script(path, &config) {
        Ok(status) => std::process::exit(status),
        Err(err) => {
            eprintln!("astra: failed to run script: {}", err);
            std::process::exit(1);
        }
    }
}

fn main() {
    let mut args = env::args().skip(1);

    if let Some(first) = args.next() {
        match first.as_str() {
            "-h" | "--help" => {
                print_usage();
                return;
            }
            "-V" | "--version" => {
                print_version();
                return;
            }
            "--theme-list" => {
                run_theme_list();
                return;
            }
            "--theme-show" => {
                run_theme_show();
                return;
            }
            "--theme-set" => {
                if let Some(theme_name) = args.next() {
                    run_theme_set(&theme_name);
                } else {
                    eprintln!("astra: --theme-set requires a theme name");
                    print_usage();
                    std::process::exit(1);
                }
                return;
            }
            "--eval" => {
                if let Some(command) = args.next() {
                    let config = config::load();
                    let status = shell::execute_line(&command, &config);
                    std::process::exit(status);
                } else {
                    eprintln!("astra: --eval requires a command string");
                    print_usage();
                    std::process::exit(1);
                }
            }
            "--config-example" => {
                print_config_example();
                return;
            }
            path => {
                let file_path = Path::new(path);
                if file_path.exists() {
                    run_script(file_path);
                } else {
                    eprintln!("Unknown argument: {}", path);
                    print_usage();
                    std::process::exit(1);
                }
            }
        }
    }

    shell::start();
}
