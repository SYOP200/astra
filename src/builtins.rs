use std::{env, process};

use crate::config::Config;
use crate::theme;

/// Executes a built-in command.
///
/// Returns:
/// - Some(exit_code) if the command was handled.
/// - None if Astra should execute it as an external program.
pub fn execute(program: &str, args: &[String], _config: &Config) -> Option<i32> {
    match program {
        "exit" => process::exit(0),

        "cd" => {
            let target = if args.is_empty() {
                dirs::home_dir().unwrap_or_default()
            } else {
                args[0].clone().into()
            };

            match env::set_current_dir(target) {
                Ok(_) => Some(0),
                Err(err) => {
                    eprintln!("astra: cd: {}", err);
                    Some(1)
                }
            }
        }

        "clear" => {
            print!("\x1B[2J\x1B[1;1H");
            Some(0)
        }

        "pwd" => match env::current_dir() {
            Ok(path) => {
                println!("{}", path.display());
                Some(0)
            }

            Err(err) => {
                eprintln!("astra: pwd: {}", err);
                Some(1)
            }
        },

        "help" => {
            println!("Astra Shell Built-in Commands");
            println!();
            println!("  help            Show this help");
            println!("  exit            Exit Astra");
            println!("  cd              Change directory");
            println!("  pwd             Print working directory");
            println!("  clear           Clear the terminal");
            println!("  version         Show Astra version");
            println!("  about           Show project information");
            println!("  theme-list      List available themes");
            println!("  theme-show      Show current theme");
            println!("  theme-set NAME  Set and save the shell theme");
            Some(0)
        }

        "version" => {
            println!("Astra v{}", env!("CARGO_PKG_VERSION"));
            Some(0)
        }

        "about" => {
            println!("Astra Shell");
            println!("Modern shell written in Rust.");
            Some(0)
        }

        "theme-list" => {
            for theme_name in theme::available() {
                println!("- {}", theme_name);
            }
            Some(0)
        }

        "theme-show" => {
            println!("Current theme: {}", _config.general.theme);
            Some(0)
        }

        "theme-set" => {
            if let Some(selected) = args.get(0) {
                let valid = theme::available()
                    .iter()
                    .any(|name| name.eq_ignore_ascii_case(selected));
                if valid {
                    let canonical = selected.to_ascii_lowercase();
                    match Config::save_theme(&canonical) {
                        Ok(_) => {
                            println!("Theme set to '{}'", canonical);
                            Some(0)
                        }
                        Err(err) => {
                            eprintln!("astra: failed to save theme: {}", err);
                            Some(1)
                        }
                    }
                } else {
                    eprintln!("astra: unknown theme '{}'. Use theme-list.", selected);
                    Some(1)
                }
            } else {
                eprintln!("astra: theme-set requires a theme name.");
                Some(1)
            }
        }

        _ => None,
    }
}
