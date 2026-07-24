use std::process::Command;

use crate::builtins;

pub fn execute(args: Vec<String>) {

    if args.is_empty() {
        return;
    }

    if builtins::run(&args) {
        return;
    }

    let result = Command::new(&args[0])
        .args(&args[1..])
        .status();

    if result.is_err() {
        println!(
            "astra: command not found: {}",
            args[0]
        );
    }
}
