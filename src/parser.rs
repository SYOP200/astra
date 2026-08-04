use crate::ast::{AstNode, Command, Pipeline, Redirect};
use crate::lexer::Token;

pub fn parse(tokens: &[Token]) -> AstNode {
    if tokens.is_empty() {
        return AstNode::Empty;
    }

    // &&
    if let Some(index) = tokens.iter().position(|t| matches!(t, Token::And)) {
        return AstNode::And(
            Box::new(parse(&tokens[..index])),
            Box::new(parse(&tokens[index + 1..])),
        );
    }

    // ||
    if let Some(index) = tokens.iter().position(|t| matches!(t, Token::Or)) {
        return AstNode::Or(
            Box::new(parse(&tokens[..index])),
            Box::new(parse(&tokens[index + 1..])),
        );
    }

    // |
    if tokens.iter().any(|t| matches!(t, Token::Pipe)) {
        return parse_pipe(tokens);
    }

    // Redirects
    if tokens.iter().any(|t| {
        matches!(
            t,
            Token::RedirectOut | Token::RedirectAppend | Token::RedirectIn | Token::RedirectErr
        )
    }) {
        return parse_redirect(tokens);
    }

    AstNode::Command(parse_command(tokens))
}

fn parse_command(tokens: &[Token]) -> Command {
    let mut words = tokens.iter().filter_map(|t| {
        if let Token::Word(word) = t {
            Some(word.clone())
        } else {
            None
        }
    });

    let program = words.next().unwrap_or_default();

    let mut command = Command::new(program);

    for arg in words {
        command.push_arg(arg);
    }

    command
}

fn parse_pipe(tokens: &[Token]) -> AstNode {
    let mut commands = Vec::new();
    let mut current = Vec::new();

    for token in tokens {
        if matches!(token, Token::Pipe) {
            commands.push(parse_command(&current));
            current.clear();
        } else {
            current.push(token.clone());
        }
    }

    if !current.is_empty() {
        commands.push(parse_command(&current));
    }

    AstNode::Pipe(Pipeline { commands })
}

fn parse_redirect(tokens: &[Token]) -> AstNode {
    let mut command_tokens = Vec::new();
    let mut redirects = Vec::new();

    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::RedirectOut => {
                if let Some(Token::Word(path)) = tokens.get(i + 1) {
                    redirects.push(Redirect::Stdout(path.into()));
                }
                i += 2;
            }

            Token::RedirectAppend => {
                if let Some(Token::Word(path)) = tokens.get(i + 1) {
                    redirects.push(Redirect::AppendStdout(path.into()));
                }
                i += 2;
            }

            Token::RedirectIn => {
                if let Some(Token::Word(path)) = tokens.get(i + 1) {
                    redirects.push(Redirect::Stdin(path.into()));
                }
                i += 2;
            }

            Token::RedirectErr => {
                if let Some(Token::Word(path)) = tokens.get(i + 1) {
                    redirects.push(Redirect::Stderr(path.into()));
                }
                i += 2;
            }

            _ => {
                command_tokens.push(tokens[i].clone());
                i += 1;
            }
        }
    }

    AstNode::Redirect {
        command: parse_command(&command_tokens),
        redirects,
    }
}
