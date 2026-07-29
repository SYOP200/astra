use rustyline::history::DefaultHistory;
use rustyline::Editor;
use std::process::{Command, Stdio};
use std::io::Write;

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
fn translate_shell_input(input_text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut child = Command::new("node")
        .arg("/path/to/translate.js")
        .env("GOOGLE_TRANSLATE_API_KEY", "YOUR_ACTUAL_API_KEY_HERE")
        .env("SHELL_TARGET_LANG", "en") // Target language variable
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Push raw user shell text into node stdin stream
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input_text.as_bytes())?;
    }

    let output = child.wait_with_output()?;
    
    if output.status.success() {
        let raw_json = String::from_utf8(output.stdout)?;
        Ok(raw_json) // Parse this JSON object inside your Rust UI framework
    } else {
        let err_json = String::from_utf8(output.stderr)?;
        Err(err_json.into())
    }
}