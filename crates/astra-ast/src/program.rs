use crate::{
    item::Item,
    stmt::Statement,
    span::{Span, Spanned},
};

/// A complete Astra source file.
///
/// This is the root node produced by the parser.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    /// Top-level declarations.
    pub items: Vec<Item>,

    /// Optional loose statements.
    ///
    /// These allow scripts such as:
    ///
    /// ```astra
    /// print("Hello")
    /// ```
    ///
    /// without requiring a main function.
    pub statements: Vec<Statement>,

    /// Location in the source file.
    pub span: Span,
}

impl Program {
    /// Creates an empty program.
    pub fn new(span: Span) -> Self {
        Self {
            items: Vec::new(),
            statements: Vec::new(),
            span,
        }
    }

    /// Adds a top-level item.
    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    /// Adds a top-level statement.
    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }

    /// Returns true if the program contains no code.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty() && self.statements.is_empty()
    }
}

impl Spanned for Program {
    fn span(&self) -> Span {
        self.span
    }
}
