use std::fs::{File, OpenOptions};
use crate::ast::*;
use std::process::{Command as ProcessCommand, Stdio};

use crate::{
    ast::{AstNode, Command, Redirect},
    builtins,
    config::Config,
    resolver,
};

pub fn execute(node: AstNode, config: &Config) -> i32 {
    match node {
        AstNode::Empty => 0,

        AstNode::Command(command) => {
            execute_command(command, config)
        }

        AstNode::Pipe(commands) => {
            execute_pipe(commands, config)
        }

        AstNode::Redirect {
            command,
            redirects,
        } => {
            execute_redirect(command, redirects, config)
        }

        AstNode::And(left, right) => {
            let status = execute(*left, config);

            if status == 0 {
                execute(*right, config)
            } else {
                status
            }
        }

        AstNode::Or(left, right) => {
            let status = execute(*left, config);

            if status != 0 {
                execute(*right, config)
            } else {
                status
            }
        }
    }
}

fn execute_command(command: Command, config: &Config) -> i32 {
    if command.program.is_empty() {
        return 0;
    }

    // Built-in commands
    if let Some(status) =
        builtins::execute(&command.program, &command.args, config)
    {
        return status;
    }

    // Resolve executable
    let executable =
        match resolver::resolve(&command.program) {
            Some(path) => path,
            None => {
                eprintln!(
                    "astra: command not found: {}",
                    command.program
                );
                return 127;
            }
        };

    let mut process = ProcessCommand::new(executable);

    process.args(&command.args);

    match process.status() {
        Ok(status) => status.code().unwrap_or(1),

        Err(error) => {
            eprintln!("astra: {}", error);
            1
        }
    }
}

fn execute_redirect(
    command: Command,
    redirects: Vec<Redirect>,
    _config: &Config,
) -> i32 {
    let executable =
        match resolver::resolve(&command.program) {
            Some(path) => path,
            None => {
                eprintln!(
                    "astra: command not found: {}",
                    command.program
                );
                return 127;
            }
        };

    let mut process = ProcessCommand::new(executable);

    process.args(&command.args);

    for redirect in redirects {
        match redirect {
            Redirect::Stdout(path) => {
                if let Ok(file) = File::create(path) {
                    process.stdout(Stdio::from(file));
                }
            }

            Redirect::AppendStdout(path) => {
                if let Ok(file) = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(path)
                {
                    process.stdout(Stdio::from(file));
                }
            }

            Redirect::Stdin(path) => {
                if let Ok(file) = File::open(path) {
                    process.stdin(Stdio::from(file));
                }
            }

            Redirect::Stderr(path) => {
                if let Ok(file) = File::create(path) {
                    process.stderr(Stdio::from(file));
                }
            }
        }
    }

    match process.status() {
        Ok(status) => status.code().unwrap_or(1),

        Err(error) => {
            eprintln!("astra: {}", error);
            1
        }
    }
}

fn execute_pipe(
    commands: Vec<Command>,
    config: &Config,
) -> i32 {
    if commands.is_empty() {
        return 0;
    }

    if commands.len() == 1 {
        return execute_command(commands[0].clone(), config);
    }

    // Placeholder until full Unix pipe implementation.
    eprintln!("astra: pipes are not implemented yet");
    1
}
