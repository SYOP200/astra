use crate::ident::Identifier;
use crate::span::{Span, Spanned};

/// A type expression in Astra source code.
#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    /// The kind of type.
    pub kind: TypeKind,

    /// Location in the source file.
    pub span: Span,
}

impl Type {
    /// Creates a new type.
    pub fn new(kind: TypeKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Creates a primitive type.
    pub fn primitive(kind: PrimitiveType) -> Self {
        Self {
            kind: TypeKind::Primitive(kind),
            span: Span::default(),
        }
    }
}

impl Spanned for Type {
    fn span(&self) -> Span {
        self.span
    }
}

/// Different kinds of Astra types.
#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    /// Built-in primitive types.
    Primitive(PrimitiveType),

    /// User-defined types.
    Named(Identifier),

    /// Array/list type.
    Array(Box<Type>),

    /// Function type.
    Function {
        parameters: Vec<Type>,
        return_type: Box<Type>,
    },

    /// Generic type parameter.
    Generic(Identifier),

    /// Unknown type used during inference.
    Unknown,
}

/// Built-in Astra types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    Int,
    Float,
    Bool,
    String,
    Char,
    Null,
    Void,
}

impl PrimitiveType {
    pub fn name(&self) -> &'static str {
        match self {
            PrimitiveType::Int => "int",
            PrimitiveType::Float => "float",
            PrimitiveType::Bool => "bool",
            PrimitiveType::String => "string",
            PrimitiveType::Char => "char",
            PrimitiveType::Null => "null",
            PrimitiveType::Void => "void",
        }
    }
}
