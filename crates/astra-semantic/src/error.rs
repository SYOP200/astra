use astra_ast::Span;
use thiserror::Error;

/// Semantic analysis errors.
///
/// These are high-level compiler errors produced during semantic analysis.
/// Most user-facing reporting should go through the diagnostics system, but
/// this type is useful when an operation must return a semantic failure.
#[derive(Debug, Error, Clone)]
pub enum SemanticError {
    #[error("undefined symbol '{name}'")]
    UndefinedSymbol {
        name: String,
        span: Span,
    },

    #[error("duplicate definition of '{name}'")]
    DuplicateSymbol {
        name: String,
        span: Span,
    },

    #[error("cannot assign to immutable variable '{name}'")]
    ImmutableAssignment {
        name: String,
        span: Span,
    },

    #[error("type mismatch: expected '{expected}', found '{found}'")]
    TypeMismatch {
        expected: String,
        found: String,
        span: Span,
    },

    #[error("function '{name}' not found")]
    UndefinedFunction {
        name: String,
        span: Span,
    },

    #[error("incorrect number of arguments for '{name}': expected {expected}, found {found}")]
    InvalidArgumentCount {
        name: String,
        expected: usize,
        found: usize,
        span: Span,
    },

    #[error("invalid return statement")]
    InvalidReturn {
        span: Span,
    },

    #[error("invalid break statement")]
    InvalidBreak {
        span: Span,
    },

    #[error("invalid continue statement")]
    InvalidContinue {
        span: Span,
    },

    #[error("{message}")]
    Custom {
        message: String,
        span: Span,
    },
}

impl SemanticError {
    /// Returns the source span associated with this error.
    pub fn span(&self) -> Span {
        match self {
            Self::UndefinedSymbol { span, .. }
            | Self::DuplicateSymbol { span, .. }
            | Self::ImmutableAssignment { span, .. }
            | Self::TypeMismatch { span, .. }
            | Self::UndefinedFunction { span, .. }
            | Self::InvalidArgumentCount { span, .. }
            | Self::InvalidReturn { span }
            | Self::InvalidBreak { span }
            | Self::InvalidContinue { span }
            | Self::Custom { span, .. } => *span,
        }
    }
}
