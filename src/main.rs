mod shell;
mod prompt;
mod parser;
mod executor;
mod builtins;
mod history;
mod config;
mod theme;
mod git;
mod completion;
mod aliases;

fn main() {
    shell::start();
}
