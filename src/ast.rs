use std::path::PathBuf;

/// A parsed shell command.
///
/// Example:
/// ```text
/// echo hello world
/// ```
#[derive(Debug, Clone)]
pub struct Command {
    /// Executable name.
    pub program: String,

    /// Arguments passed to the executable.
    pub args: Vec<String>,

    /// Input/output redirections attached to this command.
    #[allow(dead_code)]
    pub redirects: Vec<Redirect>,
}

/// Supported input/output redirections.
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

/// A pipeline of commands.
///
/// Example:
/// ```text
/// ls -la | grep rs | wc -l
/// ```
#[derive(Debug, Clone)]
pub struct Pipeline {
    pub commands: Vec<Command>,
}

/// A complete shell expression.
///
/// This is the output of the parser and the input to the executor.
#[derive(Debug, Clone)]
pub enum AstNode {
    /// Single command.
    ///
    /// ```text
    /// ls -la
    /// ```
    Command(Command),

    /// Pipeline.
    ///
    /// ```text
    /// ls | grep rs | wc -l
    /// ```
    Pipe(Pipeline),

    /// Redirects attached to a command.
    Redirect {
        command: Command,
        redirects: Vec<Redirect>,
    },

    /// Logical AND.
    ///
    /// ```text
    /// make && ./app
    /// ```
    And(Box<AstNode>, Box<AstNode>),

    /// Logical OR.
    ///
    /// ```text
    /// make || echo "Build failed"
    /// ```
    Or(Box<AstNode>, Box<AstNode>),

    /// Empty input.
    Empty,
}

#[allow(dead_code)]
impl Command {
    /// Create a new command.
    pub fn new(program: impl Into<String>) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
            redirects: Vec::new(),
        }
    }

    /// Append an argument.
    pub fn push_arg(&mut self, arg: impl Into<String>) {
        self.args.push(arg.into());
    }

    /// Append a redirect.
    pub fn push_redirect(&mut self, redirect: Redirect) {
        self.redirects.push(redirect);
    }
}

#[allow(dead_code)]
impl Pipeline {
    /// Create an empty pipeline.
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    /// Add a command to the pipeline.
    pub fn push(&mut self, command: Command) {
        self.commands.push(command);
    }

    /// Returns true if the pipeline contains no commands.
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    /// Number of commands in the pipeline.
    pub fn len(&self) -> usize {
        self.commands.len()
    }
}

#[allow(dead_code)]
impl Redirect {
    pub fn stdout(path: impl Into<PathBuf>) -> Self {
        Self::Stdout(path.into())
    }

    pub fn append_stdout(path: impl Into<PathBuf>) -> Self {
        Self::AppendStdout(path.into())
    }

    pub fn stdin(path: impl Into<PathBuf>) -> Self {
        Self::Stdin(path.into())
    }

    pub fn stderr(path: impl Into<PathBuf>) -> Self {
        Self::Stderr(path.into())
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}
