mod ast;
mod executor;
mod lexer;
mod parser;
mod resolver;

mod aliases;

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
    println!("Usage: astra [COMMAND] [ARGS] | [script.astra]");
    println!();
    println!("Global options:");
    println!("  -h, --help           Show this help message");
    println!("  -V, --version        Print release version");
    println!();
    println!("Commands:");
    println!("  theme list           List available themes");
    println!("  theme show           Show the current theme");
    println!("  theme set NAME       Set and save a theme name");
    println!("  config example       Print a sample ~/.astrarc configuration file");
    println!("  eval COMMAND         Evaluate a single command and exit");
    println!("  [script.astra]       Execute an Astra script file");
    println!();
    println!("Run without arguments to start the Astra interactive shell.");
}

fn print_version() {
    println!("Astra {}", env!("CARGO_PKG_VERSION"));
}

fn print_config_example() {
    println!("{}", config::Config::default_config_text());
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
            "Unknown theme '{}'. Use 'astra theme list' to see available themes.",
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
            "-h" | "--help" | "help" => {
                print_usage();
                return;
            }
            "-V" | "--version" | "version" => {
                print_version();
                return;
            }
            "theme" => {
                if let Some(subcommand) = args.next() {
                    match subcommand.as_str() {
                        "list" => run_theme_list(),
                        "show" => run_theme_show(),
                        "set" => {
                            if let Some(theme_name) = args.next() {
                                run_theme_set(&theme_name);
                            } else {
                                eprintln!("astra: theme set requires a theme name");
                                print_usage();
                                std::process::exit(1);
                            }
                        }
                        _ => {
                            eprintln!("Unknown theme command: {}", subcommand);
                            print_usage();
                            std::process::exit(1);
                        }
                    }
                    return;
                }
                eprintln!("Missing subcommand for theme");
                print_usage();
                std::process::exit(1);
            }
            "config" => {
                if let Some(subcommand) = args.next() {
                    if subcommand == "example" {
                        print_config_example();
                        return;
                    }
                }
                eprintln!("Unknown config command");
                print_usage();
                std::process::exit(1);
            }
            "eval" => {
                let command = args.collect::<Vec<String>>().join(" ");
                if command.is_empty() {
                    eprintln!("astra: eval requires a command string");
                    print_usage();
                    std::process::exit(1);
                }
                let config = config::load();
                let status = shell::execute_line(&command, &config);
                std::process::exit(status);
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
                let command = args.collect::<Vec<String>>().join(" ");
                if command.is_empty() {
                    eprintln!("astra: --eval requires a command string");
                    print_usage();
                    std::process::exit(1);
                }
                let config = config::load();
                let status = shell::execute_line(&command, &config);
                std::process::exit(status);
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
