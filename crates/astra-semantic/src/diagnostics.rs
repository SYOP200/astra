use astra_ast::Span;
use std::fmt;

/// Severity of a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
}

/// A single compiler diagnostic.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// Severity level.
    pub level: DiagnosticLevel,

    /// Human-readable message.
    pub message: String,

    /// Source location.
    pub span: Span,
}

impl Diagnostic {
    /// Creates a new diagnostic.
    pub fn new(
        level: DiagnosticLevel,
        message: impl Into<String>,
        span: Span,
    ) -> Self {
        Self {
            level,
            message: message.into(),
            span,
        }
    }

    /// Creates an error diagnostic.
    pub fn error(
        message: impl Into<String>,
        span: Span,
    ) -> Self {
        Self::new(DiagnosticLevel::Error, message, span)
    }

    /// Creates a warning diagnostic.
    pub fn warning(
        message: impl Into<String>,
        span: Span,
    ) -> Self {
        Self::new(DiagnosticLevel::Warning, message, span)
    }

    /// Creates a note diagnostic.
    pub fn note(
        message: impl Into<String>,
        span: Span,
    ) -> Self {
        Self::new(DiagnosticLevel::Note, message, span)
    }

    /// Returns true if this is an error.
    pub fn is_error(&self) -> bool {
        self.level == DiagnosticLevel::Error
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "{:?}: {} at {}",
            self.level,
            self.message,
            self.span
        )
    }
}

/// Collection of diagnostics produced during compilation.
#[derive(Debug, Default)]
pub struct Diagnostics {
    diagnostics: Vec<Diagnostic>,
}

impl Diagnostics {
    /// Creates an empty collection.
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    /// Adds a diagnostic.
    pub fn push(
        &mut self,
        diagnostic: Diagnostic,
    ) {
        self.diagnostics.push(diagnostic);
    }

    /// Adds an error.
    pub fn error(
        &mut self,
        message: impl Into<String>,
        span: Span,
    ) {
        self.push(Diagnostic::error(message, span));
    }

    /// Adds a warning.
    pub fn warning(
        &mut self,
        message: impl Into<String>,
        span: Span,
    ) {
        self.push(Diagnostic::warning(message, span));
    }

    /// Adds a note.
    pub fn note(
        &mut self,
        message: impl Into<String>,
        span: Span,
    ) {
        self.push(Diagnostic::note(message, span));
    }

    /// Returns all diagnostics.
    pub fn all(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Returns true if any errors were recorded.
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(Diagnostic::is_error)
    }

    /// Number of diagnostics.
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Returns true if there are no diagnostics.
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Removes all diagnostics.
    pub fn clear(&mut self) {
        self.diagnostics.clear();
    }
}
