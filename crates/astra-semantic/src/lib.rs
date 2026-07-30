//! Semantic analysis for the Astra compiler.
//!
//! This crate validates the parsed AST before lowering it into
//! later compiler stages such as HIR and MIR.
//!
//! Responsibilities include:
//! - Name resolution
//! - Scope management
//! - Symbol table construction
//! - Duplicate definition detection
//! - Undefined symbol detection
//! - Type validation (future)

pub mod analyzer;
pub mod diagnostics;
pub mod error;
pub mod scope;
pub mod symbol;
pub mod types;

pub use analyzer::SemanticAnalyzer;
pub use diagnostics::*;
pub use error::*;
pub use scope::*;
pub use symbol::*;
pub use types::*;
