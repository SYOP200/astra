use astra_ast::{Program, Span};
use astra_lexer::{Token, TokenKind};

use crate::error::ParserError;

/// Astra source parser.
pub struct Parser {
    /// Tokens produced by the lexer.
    tokens: Vec<Token>,

    /// Current position in the token stream.
    current: usize,
}

impl Parser {
    /// Creates a new parser.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    /// Parses an entire Astra program.
    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let span = if let Some(first) = self.tokens.first() {
            first.span
        } else {
            Span::default()
        };

        Ok(Program::new(span))
    }

    /// Returns the current token.
    pub fn current(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Returns the previous token.
    pub fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    /// Checks if the parser reached EOF.
    pub fn is_at_end(&self) -> bool {
        matches!(
            self.current().kind,
            TokenKind::EndOfFile
        )
    }

    /// Advances to the next token.
    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    /// Checks the current token type.
    pub fn check(
        &self,
        kind: &TokenKind,
    ) -> bool {
        &self.current().kind == kind
    }

    /// Consumes a token if it matches.
    pub fn matches(
        &mut self,
        kinds: &[TokenKind],
    ) -> bool {
        for kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }

        false
    }

    /// Requires a specific token.
    pub fn expect(
        &mut self,
        kind: TokenKind,
        message: &str,
    ) -> Result<Token, ParserError> {
        if self.check(&kind) {
            return Ok(self.advance().clone());
        }

        Err(ParserError::new(
            message,
            self.current().span,
        )
        .with_token(self.current().kind.clone()))
    }

    /// Returns the current token span.
    pub fn span(&self) -> Span {
        self.current().span
    }
}
