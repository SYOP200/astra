use crate::{
    ident::Identifier,
    literal::Literal,
    span::{Span, Spanned},
};

/// An expression in Astra source code.
#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    /// The expression kind.
    pub kind: ExprKind,

    /// Location in the source file.
    pub span: Span,
}

impl Expr {
    /// Creates a new expression.
    pub fn new(kind: ExprKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Creates an identifier expression.
    pub fn identifier(name: Identifier) -> Self {
        let span = name.span();

        Self {
            kind: ExprKind::Identifier(name),
            span,
        }
    }

    /// Creates a literal expression.
    pub fn literal(value: Literal) -> Self {
        let span = value.span();

        Self {
            kind: ExprKind::Literal(value),
            span,
        }
    }
}

impl Spanned for Expr {
    fn span(&self) -> Span {
        self.span
    }
}

/// Different expression forms supported by Astra.
#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    /// Literal value.
    Literal(Literal),

    /// Variable or identifier reference.
    Identifier(Identifier),

    /// Binary operation.
    ///
    /// Example:
    /// `a + b`
    Binary {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },

    /// Unary operation.
    ///
    /// Example:
    /// `-value`
    Unary {
        operator: UnaryOperator,
        expression: Box<Expr>,
    },

    /// Function call.
    ///
    /// Example:
    /// `print("hello")`
    Call {
        function: Box<Expr>,
        arguments: Vec<Expr>,
    },

    /// Array indexing.
    ///
    /// Example:
    /// `items[0]`
    Index {
        target: Box<Expr>,
        index: Box<Expr>,
    },

    /// Field/member access.
    ///
    /// Example:
    /// `user.name`
    Field {
        target: Box<Expr>,
        field: Identifier,
    },

    /// Array literal.
    ///
    /// Example:
    /// `[1, 2, 3]`
    Array(Vec<Expr>),

    /// Object literal.
    ///
    /// Example:
    /// `{ name: "Astra" }`
    Object(Vec<(Identifier, Expr)>),
}

/// Binary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,

    Equal,
    NotEqual,

    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    And,
    Or,
}

/// Unary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Negate,
    Not,
}
