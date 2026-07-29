use astra_ast::{
    Block, Function, Identifier, Item, ItemKind, Parameter,
    Program, Span, Statement,
};

use astra_lexer::{Token, TokenKind};

use crate::error::ParserError;

/// Astra source parser.
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    /// Parses an entire Astra program.
    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let span = self
            .tokens
            .first()
            .map(|t| t.span)
            .unwrap_or_default();

        let mut program = Program::new(span);

        while !self.is_at_end() {
            if self.matches(&[TokenKind::Fn]) {
                let function = self.parse_function()?;

                program.add_item(Item::new(
                    ItemKind::Function(function),
                    span,
                ));
            } else {
                self.advance();
            }
        }

        Ok(program)
    }

    fn parse_function(&mut self) -> Result<Function, ParserError> {
        let name = match &self.current().kind {
            TokenKind::Identifier(name) => {
                let span = self.current().span;

                self.advance();

                Identifier::new(name.clone(), span)
            }

            _ => {
                return Err(ParserError::new(
                    "expected function name",
                    self.current().span,
                ));
            }
        };

        self.expect(
            TokenKind::LeftParen,
            "expected '(' after function name",
        )?;

        let parameters = self.parse_parameters()?;

        self.expect(
            TokenKind::RightParen,
            "expected ')' after parameters",
        )?;

        let body = self.parse_block()?;

        Ok(Function {
            name,
            parameters,
            return_type: None,
            body,
        })
    }

    fn parse_parameters(
        &mut self,
    ) -> Result<Vec<Parameter>, ParserError> {
        let mut parameters = Vec::new();

        while !self.check(&TokenKind::RightParen)
            && !self.is_at_end()
        {
            let name = match &self.current().kind {
                TokenKind::Identifier(value) => {
                    let span = self.current().span;

                    self.advance();

                    Identifier::new(value.clone(), span)
                }

                _ => {
                    return Err(ParserError::new(
                        "expected parameter name",
                        self.current().span,
                    ));
                }
            };

            self.expect(
                TokenKind::Colon,
                "expected ':' after parameter name",
            )?;

            // Temporary type placeholder.
            // Type parsing is ASTRA-004B.
            self.advance();

            parameters.push(Parameter {
                name,
                ty: astra_ast::Type::new(
                    astra_ast::TypeKind::Unknown,
                    self.previous().span,
                ),
                span: self.previous().span,
            });

            if !self.matches(&[TokenKind::Comma]) {
                break;
            }
        }

        Ok(parameters)
    }

    fn parse_block(&mut self) -> Result<Block, ParserError> {
        self.expect(
            TokenKind::LeftBrace,
            "expected '{'",
        )?;

        let span = self.previous().span;

        let mut block = Block::new(span);

        while !self.check(&TokenKind::RightBrace)
            && !self.is_at_end()
        {
            // Statements are ASTRA-004C.
            self.advance();
        }

        self.expect(
            TokenKind::RightBrace,
            "expected '}'",
        )?;

        Ok(block)
    }

    pub fn current(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    pub fn is_at_end(&self) -> bool {
        matches!(
            self.current().kind,
            TokenKind::EndOfFile
        )
    }

    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    pub fn check(&self, kind: &TokenKind) -> bool {
        &self.current().kind == kind
    }

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

    pub fn expect(
        &mut self,
        kind: TokenKind,
        message: &str,
    ) -> Result<Token, ParserError> {
        if self.check(&kind) {
            return Ok(self.advance().clone());
        }

        Err(
            ParserError::new(
                message,
                self.current().span,
            )
            .with_token(self.current().kind.clone()),
        )
    }
}
