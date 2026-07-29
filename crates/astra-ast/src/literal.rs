use std::fmt;

use crate::span::{Span, Spanned};

/// A literal value written directly in Astra source code.
#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    /// The literal value.
    pub kind: LiteralKind,

    /// Location in the source file.
    pub span: Span,
}

impl Literal {
    /// Creates a new literal.
    pub fn new(kind: LiteralKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Returns the literal type name.
    pub fn type_name(&self) -> &'static str {
        match self.kind {
            LiteralKind::Integer(_) => "int",
            LiteralKind::Float(_) => "float",
            LiteralKind::String(_) => "string",
            LiteralKind::Boolean(_) => "bool",
            LiteralKind::Character(_) => "char",
            LiteralKind::Null => "null",
        }
    }
}

/// Supported literal values.
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralKind {
    /// Signed integer.
    Integer(i64),

    /// Floating point number.
    Float(f64),

    /// String literal.
    String(String),

    /// Boolean literal.
    Boolean(bool),

    /// Character literal.
    Character(char),

    /// Empty/null value.
    Null,
}

impl Spanned for Literal {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for LiteralKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralKind::Integer(value) => {
                write!(f, "{value}")
            }

            LiteralKind::Float(value) => {
                write!(f, "{value}")
            }

            LiteralKind::String(value) => {
                write!(f, "\"{value}\"")
            }

            LiteralKind::Boolean(value) => {
                write!(f, "{value}")
            }

            LiteralKind::Character(value) => {
                write!(f, "'{value}'")
            }

            LiteralKind::Null => {
                write!(f, "null")
            }
        }
    }
}
