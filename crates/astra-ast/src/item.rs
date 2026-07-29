use crate::{
    expr::Expr,
    ident::Identifier,
    stmt::Block,
    span::{Span, Spanned},
    ty::Type,
};

/// A top-level item in an Astra source file.
#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    /// The item kind.
    pub kind: ItemKind,

    /// Location in the source file.
    pub span: Span,
}

impl Item {
    /// Creates a new item.
    pub fn new(kind: ItemKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl Spanned for Item {
    fn span(&self) -> Span {
        self.span
    }
}

/// Different top-level declarations in Astra.
#[derive(Debug, Clone, PartialEq)]
pub enum ItemKind {
    /// Function declaration.
    ///
    /// Example:
    /// ```astra
    /// fn add(a: int, b: int) -> int {
    ///     return a + b
    /// }
    /// ```
    Function(Function),

    /// Struct declaration.
    ///
    /// Example:
    /// ```astra
    /// struct User {
    ///     name: string
    /// }
    /// ```
    Struct(Struct),

    /// Constant declaration.
    ///
    /// Example:
    /// ```astra
    /// const VERSION = "1.0"
    /// ```
    Const {
        name: Identifier,
        ty: Option<Type>,
        value: Expr,
    },

    /// Import declaration.
    ///
    /// Example:
    /// ```astra
    /// import fs
    /// ```
    Import {
        module: Identifier,
    },

    /// Module declaration.
    ///
    /// Example:
    /// ```astra
    /// module network
    /// ```
    Module {
        name: Identifier,
    },
}

/// A function declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    /// Function name.
    pub name: Identifier,

    /// Function parameters.
    pub parameters: Vec<Parameter>,

    /// Return type.
    pub return_type: Option<Type>,

    /// Function body.
    pub body: Block,
}

/// A function parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    /// Parameter name.
    pub name: Identifier,

    /// Parameter type.
    pub ty: Type,

    /// Location in source.
    pub span: Span,
}

impl Spanned for Parameter {
    fn span(&self) -> Span {
        self.span
    }
}

/// A struct declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    /// Struct name.
    pub name: Identifier,

    /// Struct fields.
    pub fields: Vec<Field>,

    /// Location in source.
    pub span: Span,
}

/// A struct field.
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    /// Field name.
    pub name: Identifier,

    /// Field type.
    pub ty: Type,

    /// Location in source.
    pub span: Span,
}

impl Spanned for Field {
    fn span(&self) -> Span {
        self.span
    }
}

impl Spanned for Struct {
    fn span(&self) -> Span {
        self.span
    }
}
