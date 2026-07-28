use std::path::PathBuf;

/// A parsed shell command.
///
/// Example:
/// echo hello world
#[derive(Debug, Clone)]
pub struct Command {
    /// Executable name.
    pub program: String,

    /// Arguments passed to the executable.
    pub args: Vec<String>,
}

/// Supported output/input redirections.
#[derive(Debug, Clone)]
pub enum Redirect {
    /// >
    Stdout(PathBuf),

    /// >>
    AppendStdout(PathBuf),

    /// <
    Stdin(PathBuf),

    /// 2>
    Stderr(PathBuf),
}

/// A complete shell expression.
///
/// This becomes the output of the parser.
#[derive(Debug, Clone)]
pub enum AstNode {
    /// Single command.
    ///
    /// ls -la
    Command(Command),

    /// cmd1 | cmd2
    Pipe(Vec<Command>),

    /// cmd > file
    Redirect {
        command: Command,
        redirects: Vec<Redirect>,
    },

    /// cmd1 && cmd2
    And(Box<AstNode>, Box<AstNode>),

    /// cmd1 || cmd2
    Or(Box<AstNode>, Box<AstNode>),

    /// Empty input.
    Empty,
}

impl Command {
    pub fn new(program: String) -> Self {
        Self {
            program,
            args: Vec::new(),
        }
    }

    pub fn push_arg(&mut self, arg: String) {
        self.args.push(arg);
    }
}
