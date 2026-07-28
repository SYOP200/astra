use rustyline::history::DefaultHistory;
use rustyline::Editor;

use crate::{
    completion::AstraCompleter,
    config,
    executor,
    history,
    lexer,
    parser,
    prompt,
};

pub fn start() {
    let config = config::load();

    let mut readline =
        Editor::<AstraCompleter, DefaultHistory>::new()
            .expect("failed to initialize terminal");

    readline.set_helper(Some(AstraCompleter::new()));

    history::load(&mut readline);

    loop {
        let prompt = prompt::render(&config);

        let input = readline.readline(&prompt);

        match input {
            Ok(command) => {
                let command = command.trim();

                if command.is_empty() {
                    continue;
                }

                let _ = readline.add_history_entry(command);

                // Step 1: Tokenize
                let tokens = lexer::tokenize(command);

                // Step 2: Parse
                let ast = parser::parse(&tokens);

                // Step 3: Execute
                executor::execute(ast, &config);
            }

            Err(_) => {
                println!();
                break;
            }
        }
    }

    history::save(&mut readline);
}
