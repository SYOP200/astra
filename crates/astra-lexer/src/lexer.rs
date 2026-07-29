use astra_ast::Span;

use crate::token::{Token, TokenKind};

/// Astra source lexer.
pub struct Lexer {
    source: Vec<char>,
    position: usize,
}

impl Lexer {
    /// Creates a new lexer.
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
        }
    }

    /// Tokenizes the entire source file.
    pub fn tokenize(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.skip_whitespace();

            if self.is_at_end() {
                break;
            }

            let start = self.position;

            let kind = self.next_token();

            tokens.push(Token::new(
                kind,
                Span::new(start as u32, self.position as u32),
            ));
        }

        tokens.push(Token::new(
            TokenKind::EndOfFile,
            Span::new(
                self.position as u32,
                self.position as u32,
            ),
        ));

        tokens
    }

    fn next_token(&mut self) -> TokenKind {
        let c = self.advance();

        match c {
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,

            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,

            '[' => TokenKind::LeftBracket,
            ']' => TokenKind::RightBracket,

            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semicolon,

            '+' => TokenKind::Plus,

            '-' => {
                if self.match_char('>') {
                    TokenKind::Arrow
                } else {
                    TokenKind::Minus
                }
            }

            '*' => TokenKind::Star,

            '/' => TokenKind::Slash,

            '%' => TokenKind::Percent,

            '=' => {
                if self.match_char('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                }
            }

            '!' => {
                if self.match_char('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                }
            }

            '<' => {
                if self.match_char('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }

            '>' => {
                if self.match_char('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }

            '&' => {
                if self.match_char('&') {
                    TokenKind::AndAnd
                } else {
                    TokenKind::AndAnd
                }
            }

            '|' => {
                if self.match_char('|') {
                    TokenKind::OrOr
                } else {
                    TokenKind::OrOr
                }
            }

            '"' => self.string(),

            '\'' => self.character(),

            c if c.is_ascii_digit() => self.number(),

            c if is_identifier_start(c) => self.identifier(),

            _ => TokenKind::Newline,
        }
    }

    fn identifier(&mut self) -> TokenKind {
        while !self.is_at_end()
            && is_identifier_part(self.peek())
        {
            self.advance();
        }

        let value: String = self.source
            .iter()
            .skip(self.position)
            .collect();

        match value.as_str() {
            "fn" => TokenKind::Fn,
            "let" => TokenKind::Let,
            "const" => TokenKind::Const,
            "struct" => TokenKind::Struct,
            "import" => TokenKind::Import,
            "module" => TokenKind::Module,
            "return" => TokenKind::Return,

            "if" => TokenKind::If,
            "else" => TokenKind::Else,

            "while" => TokenKind::While,
            "loop" => TokenKind::Loop,

            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,

            "true" => TokenKind::Boolean(true),
            "false" => TokenKind::Boolean(false),
            "null" => TokenKind::Null,

            "int" => TokenKind::TypeInt,
            "float" => TokenKind::TypeFloat,
            "bool" => TokenKind::TypeBool,
            "string" => TokenKind::TypeString,
            "char" => TokenKind::TypeChar,
            "void" => TokenKind::TypeVoid,

            _ => TokenKind::Identifier(value),
        }
    }

    fn number(&mut self) -> TokenKind {
        let start = self.position - 1;
        let mut floating = false;

        while !self.is_at_end()
            && self.peek().is_ascii_digit()
        {
            self.advance();
        }

        if self.peek() == '.'
            && self
                .peek_next()
                .is_ascii_digit()
        {
            floating = true;
            self.advance();

            while !self.is_at_end()
                && self.peek().is_ascii_digit()
            {
                self.advance();
            }
        }

        let value: String = self.source[start..self.position]
            .iter()
            .collect();

        if floating {
            TokenKind::Float(
                value.parse().unwrap_or(0.0),
            )
        } else {
            TokenKind::Integer(
                value.parse().unwrap_or(0),
            )
        }
    }

    fn string(&mut self) -> TokenKind {
        let mut value = String::new();

        while !self.is_at_end()
            && self.peek() != '"'
        {
            value.push(self.advance());
        }

        self.advance();

        TokenKind::String(value)
    }

    fn character(&mut self) -> TokenKind {
        let value = self.advance();

        self.advance();

        TokenKind::Character(value)
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end()
            && self.peek().is_whitespace()
        {
            self.advance();
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.position];
        self.position += 1;
        c
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end()
            || self.source[self.position] != expected
        {
            return false;
        }

        self.position += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.position]
        }
    }

    fn peek_next(&self) -> char {
        if self.position + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.position + 1]
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }
}

fn is_identifier_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_identifier_part(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}
