use rustyline::history::DefaultHistory;
use rustyline::Editor;

use crate::{completion::AstraCompleter, config, executor, history, lexer, parser, prompt};

pub fn start() {
    let mut readline =
        Editor::<AstraCompleter, DefaultHistory>::new().expect("failed to initialize terminal");

    readline.set_helper(Some(AstraCompleter::new()));

    let mut config = config::load();
    history::load(&mut readline, &config);

    loop {
        config = config::load();
        let prompt = prompt::render(&config);

        match readline.readline(&prompt) {
            Ok(command) => {
                let command = command.trim();

                if command.is_empty() {
                    continue;
                }

                let _ = readline.add_history_entry(command);
                execute_line(command, &config);
            }

            Err(_) => {
                println!();
                break;
            }
        }
    }

    history::save(&mut readline, &config);
}

pub fn execute_line(line: &str, config: &config::Config) -> i32 {
    let tokens = lexer::tokenize(line);
    let ast = parser::parse(&tokens);
    executor::execute(ast, config)
}

pub fn run_script(path: &std::path::Path, config: &config::Config) -> Result<i32, std::io::Error> {
    let contents = std::fs::read_to_string(path)?;
    let mut status = 0;

    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        status = execute_line(line, config);
    }

    Ok(status)
}
