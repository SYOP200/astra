mod ast;
mod lexer;
mod parser;
mod executor;
mod resolver;

mod shell;
mod prompt;
mod builtins;
mod history;
mod config;
mod theme;
mod git;
mod completion;

fn main() {
    shell::start();
}
