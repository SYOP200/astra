use astra_ast::{Identifier, Type};
use std::fmt;

/// What kind of symbol this is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Variable,
    Constant,
    Function,
    Parameter,
    Struct,
    Enum,
    Trait,
    Module,
}

/// A symbol in the semantic symbol table.
#[derive(Debug, Clone)]
pub struct Symbol {
    /// Symbol name.
    pub name: Identifier,

    /// What this symbol represents.
    pub kind: SymbolKind,

    /// Type of the symbol.
    pub ty: Option<Type>,

    /// Scope depth where this symbol exists.
    pub scope_depth: usize,

    /// Whether this symbol can be reassigned.
    pub mutable: bool,
}

impl Symbol {
    pub fn new(
        name: Identifier,
        kind: SymbolKind,
        ty: Option<Type>,
        scope_depth: usize,
        mutable: bool,
    ) -> Self {
        Self {
            name,
            kind,
            ty,
            scope_depth,
            mutable,
        }
    }

    pub fn is_function(&self) -> bool {
        self.kind == SymbolKind::Function
    }

    pub fn is_variable(&self) -> bool {
        self.kind == SymbolKind::Variable
    }

    pub fn is_type(&self) -> bool {
        matches!(
            self.kind,
            SymbolKind::Struct
                | SymbolKind::Enum
                | SymbolKind::Trait
        )
    }
}

impl fmt::Display for Symbol {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{:?} {}", self.kind, self.name)
    }
}
