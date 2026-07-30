use std::collections::HashMap;

use astra_ast::Identifier;

use crate::symbol::Symbol;

/// A lexical scope.
///
/// Scopes form a tree. Each scope knows its parent and stores the
/// symbols declared directly within it.
#[derive(Debug, Clone)]
pub struct Scope {
    /// Parent scope index.
    pub parent: Option<usize>,

    /// Nesting depth.
    pub depth: usize,

    /// Symbols declared in this scope.
    symbols: HashMap<String, Symbol>,
}

impl Scope {
    /// Creates a new scope.
    pub fn new(parent: Option<usize>, depth: usize) -> Self {
        Self {
            parent,
            depth,
            symbols: HashMap::new(),
        }
    }

    /// Returns true if this scope contains no symbols.
    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }

    /// Number of symbols in this scope.
    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    /// Insert a symbol.
    ///
    /// Returns the previous symbol if one already existed.
    pub fn insert(&mut self, symbol: Symbol) -> Option<Symbol> {
        self.symbols
            .insert(symbol.name.to_string(), symbol)
    }

    /// Returns true if this scope contains the identifier.
    pub fn contains(&self, ident: &Identifier) -> bool {
        self.symbols.contains_key(&ident.to_string())
    }

    /// Lookup an identifier inside this scope only.
    pub fn get(&self, ident: &Identifier) -> Option<&Symbol> {
        self.symbols.get(&ident.to_string())
    }

    /// Mutable lookup.
    pub fn get_mut(
        &mut self,
        ident: &Identifier,
    ) -> Option<&mut Symbol> {
        self.symbols.get_mut(&ident.to_string())
    }

    /// Remove a symbol.
    pub fn remove(
        &mut self,
        ident: &Identifier,
    ) -> Option<Symbol> {
        self.symbols.remove(&ident.to_string())
    }

    /// Remove all symbols.
    pub fn clear(&mut self) {
        self.symbols.clear();
    }

    /// Iterate over symbols.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&String, &Symbol)> {
        self.symbols.iter()
    }

    /// Mutable iterator.
    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = (&String, &mut Symbol)> {
        self.symbols.iter_mut()
    }
}
