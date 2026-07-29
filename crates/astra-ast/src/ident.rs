use crate::span::{Span, Spanned};

/// A named identifier in Astra source code.
///
/// Examples:
/// - variable names
/// - function names
/// - type names
/// - module names
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    /// The identifier text.
    pub name: String,

    /// Location in the source file.
    pub span: Span,
}

impl Identifier {
    /// Creates a new identifier.
    pub fn new(name: impl Into<String>, span: Span) -> Self {
        Self {
            name: name.into(),
            span,
        }
    }

    /// Returns the identifier text.
    pub fn as_str(&self) -> &str {
        &self.name
    }

    /// Returns true if the identifier is empty.
    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
    }
}

impl Spanned for Identifier {
    fn span(&self) -> Span {
        self.span
    }
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Self {
            name: value,
            span: Span::default(),
        }
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self {
            name: value.to_string(),
            span: Span::default(),
        }
    }
}
