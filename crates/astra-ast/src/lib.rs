//! Astra Abstract Syntax Tree
//!
//! This crate defines the syntax tree used by the parser,
//! type checker, compiler, and language server.

pub mod span;
pub mod ident;
pub mod literal;
pub mod expr;
pub mod stmt;
pub mod item;
pub mod ty;
pub mod program;

pub use span::*;
pub use ident::*;
pub use literal::*;
pub use expr::*;
pub use stmt::*;
pub use item::*;
pub use ty::*;
pub use program::*;
