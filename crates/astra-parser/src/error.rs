use astra_ast::Span;
use astra_lexer::TokenKind;
use std::fmt;

/// Parser errors produced while converting tokens into AST nodes.
#[derive(Debug, Clone)]
pub struct ParserError {
    /// Error message.
    pub message: String,

    /// Location of the error.
    pub span: Span,

    /// Token that caused the error.
    pub found: Option<TokenKind>,
}

impl ParserError {
    /// Creates a new parser error.
    pub fn new(
        message: impl Into<String>,
        span: Span,
    ) -> Self {
        Self {
            message: message.into(),
            span,
            found: None,
        }
    }

    /// Creates an unexpected token error.
    pub fn unexpected(
        token: TokenKind,
        span: Span,
    ) -> Self {
        Self {
            message: "unexpected token".to_string(),
            span,
            found: Some(token),
        }
    }

    /// Adds the offending token.
    pub fn with_token(
        mut self,
        token: TokenKind,
    ) -> Self {
        self.found = Some(token);
        self
    }
}

impl fmt::Display for ParserError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match &self.found {
            Some(token) => {
                write!(
                    f,
                    "{} at {} (found {:?})",
                    self.message,
                    self.span,
                    token
                )
            }

            None => {
                write!(
                    f,
                    "{} at {}",
                    self.message,
                    self.span
                )
            }
        }
    }
}

impl std::error::Error for ParserError {}
