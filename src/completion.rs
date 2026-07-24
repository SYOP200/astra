use rustyline::{
    completion::{
        Completer,
        Pair
    },
    highlight::Highlighter,
    hint::Hinter,
    validate::Validator,
    Context,
    Helper
};

use std::fs;

pub struct AstraCompleter;

impl AstraCompleter {

    pub fn new() -> Self {
        Self
    }

}


impl Helper for AstraCompleter {}

impl Hinter for AstraCompleter {
    type Hint = String;
}


impl Highlighter for AstraCompleter {}

impl Validator for AstraCompleter {}


impl Completer for AstraCompleter {

    type Candidate = Pair;


    fn complete(
        &self,
        line: &str,
        pos: usize,
        _: &Context<'_>
    ) -> rustyline::Result<(usize, Vec<Pair>)> {


        let start = line[..pos]
            .rfind(' ')
            .map(|i| i + 1)
            .unwrap_or(0);


        let current = &line[start..pos];


        let mut results = Vec::new();


        let commands = [
            "cd",
            "pwd",
            "clear",
            "exit",
            "astra"
        ];


        for command in commands {

            if command.starts_with(current) {

                results.push(
                    Pair {
                        display: command.into(),
                        replacement: command.into()
                    }
                );
            }
        }


        if let Ok(entries) = fs::read_dir(".") {

            for entry in entries.flatten() {

                let name =
                    entry.file_name()
                    .to_string_lossy()
                    .to_string();


                if name.starts_with(current) {

                    results.push(
                        Pair {
                            display: name.clone(),
                            replacement: name
                        }
                    );
                }
            }
        }


        Ok((start, results))
    }
}
