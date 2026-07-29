//! src/ast.rs
//!
//! Abstract Syntax Tree (AST) for Astra Shell.
//!
//! The lexer converts source text into tokens.
//! The parser converts tokens into this AST.
//! The executor consumes this AST.
//!
//! This module intentionally contains no parsing logic.

/// Byte span within the original input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Pipeline(Pipeline),
    Sequence(Vec<Statement>),
    And(Box<Statement>, Box<Statement>),
    Or(Box<Statement>, Box<Statement>),
    Subshell(Box<Program>),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Pipeline {
    pub commands: Vec<Command>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub executable: Argument,
    pub arguments: Vec<Argument>,
    pub redirects: Vec<Redirect>,
    pub assignments: Vec<Assignment>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Argument {
    /// ls
    Literal(String),

    /// "$HOME"
    Quoted(String),

    /// $HOME
    Variable(String),

    /// $(pwd)
    CommandSubstitution(Box<Program>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Redirect {
    pub kind: RedirectKind,
    pub target: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedirectKind {
    Input,
    Output,
    Append,
    Error,
    ErrorAppend,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub name: String,
    pub value: String,
}

impl Command {
    pub fn new(executable: impl Into<String>) -> Self {
        Self {
            executable: Argument::Literal(executable.into()),
            arguments: Vec::new(),
            redirects: Vec::new(),
            assignments: Vec::new(),
            span: Span::default(),
        }
    }
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}
