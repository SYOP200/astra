use rustyline::DefaultEditor;

use crate::{
    completion::AstraCompleter,
    config,
    executor,
    history,
    parser,
    prompt
};

pub fn start() {

    let config = config::load();

    let mut readline = DefaultEditor::new()
        .expect("failed to initialize terminal");


    readline.set_helper(
        Some(AstraCompleter::new())
    );


    history::load(&mut readline);


    loop {

        let prompt = prompt::render(&config);


        let input = readline.readline(&prompt);


        match input {

            Ok(command) => {

                if command.trim().is_empty() {
                    continue;
                }


                let _ = readline.add_history_entry(
                    command.as_str()
                );


                let args =
                    parser::parse(&command);


                executor::execute(args);
            }


            Err(_) => {
                println!();
                break;
            }
        }
    }


    history::save(&mut readline);
}
