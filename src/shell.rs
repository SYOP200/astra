use rustyline::history::DefaultHistory;
use rustyline::Editor;

use crate::{completion::AstraCompleter, config, executor, history, lexer, parser, prompt};

pub fn start() {
    let config = config::load();

    let mut readline =
        Editor::<AstraCompleter, DefaultHistory>::new().expect("failed to initialize terminal");

    readline.set_helper(Some(AstraCompleter::new()));

    // Load history
    history::load(&mut readline, &config);

    loop {
        let prompt = prompt::render(&config);

        match readline.readline(&prompt) {
            Ok(command) => {
                let command = command.trim();

                if command.is_empty() {
                    continue;
                }

                let _ = readline.add_history_entry(command);

                // Tokenize
                let tokens = lexer::tokenize(command);

                // Parse
                let ast = parser::parse(&tokens);

                // Execute
                executor::execute(ast, &config);
            }

            Err(_) => {
                println!();
                break;
            }
        }
    }

    // Save history
    history::save(&mut readline, &config);
}
