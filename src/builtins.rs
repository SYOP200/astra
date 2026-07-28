use std::{
    env,
    process,
};

use crate::config::Config;

/// Executes a built-in command.
///
/// Returns:
/// - Some(exit_code) if the command was handled.
/// - None if Astra should execute it as an external program.
pub fn execute(
    program: &str,
    args: &[String],
    _config: &Config,
) -> Option<i32> {
    match program {
        "exit" => {
            process::exit(0);
        }

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

        "pwd" => {
            match env::current_dir() {
                Ok(path) => {
                    println!("{}", path.display());
                    Some(0)
                }

                Err(err) => {
                    eprintln!("astra: pwd: {}", err);
                    Some(1)
                }
            }
        }

        "help" => {
            println!("Astra Shell Built-in Commands");
            println!();
            println!("  help      Show this help");
            println!("  exit      Exit Astra");
            println!("  cd        Change directory");
            println!("  pwd       Print working directory");
            println!("  clear     Clear the terminal");
            Some(0)
        }

        "version" => {
            println!("Astra v0.5.0-dev");
            Some(0)
        }

        "about" => {
            println!("Astra Shell");
            println!("Modern shell written in Rust.");
            Some(0)
        }

        _ => None,
    }
}
