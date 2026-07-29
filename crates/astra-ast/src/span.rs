//! Source code location tracking.

use std::fmt;

/// A byte offset into a source file.
pub type BytePos = u32;

/// A span representing a region in a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Span {
    pub start: BytePos,
    pub end: BytePos,
}

impl Span {
    /// Creates a new span.
    pub const fn new(start: BytePos, end: BytePos) -> Self {
        Self { start, end }
    }

    /// Returns an empty span.
    pub const fn empty() -> Self {
        Self { start: 0, end: 0 }
    }

    /// Length of the span in bytes.
    pub const fn len(self) -> BytePos {
        self.end.saturating_sub(self.start)
    }

    /// Returns true if the span is empty.
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }

    /// Returns true if this span contains a byte offset.
    pub const fn contains(self, pos: BytePos) -> bool {
        pos >= self.start && pos < self.end
    }

    /// Returns a span covering both spans.
    pub const fn merge(self, other: Span) -> Span {
        Span {
            start: if self.start < other.start {
                self.start
            } else {
                other.start
            },
            end: if self.end > other.end {
                self.end
            } else {
                other.end
            },
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

/// Any AST node that originates from source code should implement this trait.
pub trait Spanned {
    fn span(&self) -> Span;
}
