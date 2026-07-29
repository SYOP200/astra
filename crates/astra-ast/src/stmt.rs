use crate::{
    expr::Expr,
    ident::Identifier,
    span::{Span, Spanned},
    ty::Type,
};

/// A statement in Astra source code.
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    /// The statement kind.
    pub kind: StatementKind,

    /// Location in the source file.
    pub span: Span,
}

impl Statement {
    /// Creates a new statement.
    pub fn new(kind: StatementKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl Spanned for Statement {
    fn span(&self) -> Span {
        self.span
    }
}

/// Different statement forms supported by Astra.
#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind {
    /// Variable declaration.
    ///
    /// Example:
    /// `let name = "Astra"`
    Let {
        name: Identifier,
        ty: Option<Type>,
        value: Option<Expr>,
    },

    /// Expression statement.
    ///
    /// Example:
    /// `print("hello")`
    Expression(Expr),

    /// Return statement.
    ///
    /// Example:
    /// `return value`
    Return(Option<Expr>),

    /// Conditional statement.
    ///
    /// Example:
    /// `if condition { }`
    If {
        condition: Expr,
        then_branch: Block,
        else_branch: Option<Block>,
    },

    /// While loop.
    ///
    /// Example:
    /// `while condition { }`
    While {
        condition: Expr,
        body: Block,
    },

    /// Infinite loop.
    ///
    /// Example:
    /// `loop { }`
    Loop {
        body: Block,
    },

    /// A scoped block of statements.
    Block(Block),

    /// Break from a loop.
    Break,

    /// Continue a loop.
    Continue,
}

/// A group of statements surrounded by braces.
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    /// Statements inside the block.
    pub statements: Vec<Statement>,

    /// Location in the source file.
    pub span: Span,
}

impl Block {
    /// Creates an empty block.
    pub fn new(span: Span) -> Self {
        Self {
            statements: Vec::new(),
            span,
        }
    }

    /// Adds a statement to the block.
    pub fn push(&mut self, statement: Statement) {
        self.statements.push(statement);
    }

    /// Returns true if the block has no statements.
    pub fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }
}

impl Spanned for Block {
    fn span(&self) -> Span {
        self.span
    }
}
