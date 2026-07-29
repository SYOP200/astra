use astra_ast::Span;

/// A token produced by the Astra lexer.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// Token type.
    pub kind: TokenKind,

    /// Location in source code.
    pub span: Span,
}

impl Token {
    /// Creates a new token.
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// All token types supported by Astra.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // -----------------
    // Literals
    // -----------------

    Integer(i64),

    Float(f64),

    String(String),

    Character(char),

    Boolean(bool),

    Null,


    // -----------------
    // Identifiers
    // -----------------

    Identifier(String),


    // -----------------
    // Keywords
    // -----------------

    Fn,

    Let,

    Const,

    Struct,

    Import,

    Module,

    Return,

    If,

    Else,

    While,

    Loop,

    Break,

    Continue,


    // -----------------
    // Types
    // -----------------

    TypeInt,

    TypeFloat,

    TypeBool,

    TypeString,

    TypeChar,

    TypeVoid,


    // -----------------
    // Operators
    // -----------------

    Plus,

    Minus,

    Star,

    Slash,

    Percent,


    Equal,

    EqualEqual,

    Bang,

    BangEqual,


    Less,

    LessEqual,

    Greater,

    GreaterEqual,


    AndAnd,

    OrOr,


    Arrow,


    // -----------------
    // Punctuation
    // -----------------

    LeftParen,

    RightParen,

    LeftBrace,

    RightBrace,

    LeftBracket,

    RightBracket,


    Comma,

    Dot,

    Colon,

    Semicolon,


    // -----------------
    // Special
    // -----------------

    Newline,

    EndOfFile,
}

impl TokenKind {
    /// Returns true if this token is a keyword.
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            TokenKind::Fn
                | TokenKind::Let
                | TokenKind::Const
                | TokenKind::Struct
                | TokenKind::Import
                | TokenKind::Module
                | TokenKind::Return
                | TokenKind::If
                | TokenKind::Else
                | TokenKind::While
                | TokenKind::Loop
                | TokenKind::Break
                | TokenKind::Continue
        )
    }
}
