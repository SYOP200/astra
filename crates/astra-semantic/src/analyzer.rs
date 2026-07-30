use std::collections::HashMap;

use astra_ast::*;

use crate::{
    diagnostics::*,
    error::*,
    scope::*,
    symbol::*,
    types::*,
};

/// Result of semantic analysis.
#[derive(Debug)]
pub struct SemanticResult {
    pub diagnostics: Diagnostics,
    pub scopes: Vec<Scope>,
}

impl SemanticResult {
    pub fn success(&self) -> bool {
        !self.diagnostics.has_errors()
    }
}

/// Main semantic analyzer.
pub struct SemanticAnalyzer {
    scopes: Vec<Scope>,
    current_scope: usize,

    diagnostics: Diagnostics,

    functions: HashMap<String, Symbol>,
    types: HashMap<String, Symbol>,
}

impl SemanticAnalyzer {
    /// Creates a new analyzer.
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new(None, 0)],
            current_scope: 0,

            diagnostics: Diagnostics::new(),

            functions: HashMap::new(),
            types: HashMap::new(),
        }
    }

    /// Analyze an AST.
    pub fn analyze(
        mut self,
        root: &AstNode,
    ) -> SemanticResult {
        self.visit_node(root);

        SemanticResult {
            diagnostics: self.diagnostics,
            scopes: self.scopes,
        }
    }

    fn visit_node(
        &mut self,
        node: &AstNode,
    ) {
        match node {
            AstNode::Empty => {}

            AstNode::Command(command) => {
                self.visit_command(command);
            }

            AstNode::Pipe(pipe) => {
                for command in &pipe.commands {
                    self.visit_command(command);
                }
            }

            AstNode::And(left, right) => {
                self.visit_node(left);
                self.visit_node(right);
            }

            AstNode::Or(left, right) => {
                self.visit_node(left);
                self.visit_node(right);
            }

            AstNode::Sequence(nodes) => {
                for node in nodes {
                    self.visit_node(node);
                }
            }
        }
    }

    fn visit_command(
        &mut self,
        command: &Command,
    ) {
        if command.program.is_empty() {
            return;
        }

        let symbol = Symbol::new(
            Identifier::new(command.program.clone()),
            SymbolKind::Function,
            None,
            self.current_scope,
            false,
        );

        let scope = &mut self.scopes[self.current_scope];

        if scope.contains(&symbol.name) {
            self.diagnostics.error(
                format!(
                    "duplicate definition '{}'",
                    symbol.name
                ),
                Span::default(),
            );

            return;
        }

        scope.insert(symbol.clone());

        self.functions
            .insert(symbol.name.to_string(), symbol);
    }

    pub fn enter_scope(&mut self) {
        let parent = self.current_scope;

        let depth = self.scopes[parent].depth + 1;

        self.scopes.push(Scope::new(
            Some(parent),
            depth,
        ));

        self.current_scope = self.scopes.len() - 1;
    }

    pub fn leave_scope(&mut self) {
        if let Some(parent) =
            self.scopes[self.current_scope].parent
        {
            self.current_scope = parent;
        }
    }

    pub fn define(
        &mut self,
        symbol: Symbol,
    ) -> Result<(), SemanticError> {
        let scope =
            &mut self.scopes[self.current_scope];

        if scope.contains(&symbol.name) {
            return Err(
                SemanticError::DuplicateSymbol {
                    name: symbol.name.to_string(),
                    span: Span::default(),
                },
            );
        }

        scope.insert(symbol);

        Ok(())
    }

    pub fn resolve(
        &self,
        name: &Identifier,
    ) -> Option<&Symbol> {
        let mut scope = Some(self.current_scope);

        while let Some(index) = scope {
            let current = &self.scopes[index];

            if let Some(symbol) = current.get(name) {
                return Some(symbol);
            }

            scope = current.parent;
        }

        None
    }

    pub fn diagnostics(&self) -> &Diagnostics {
        &self.diagnostics
    }

    pub fn scopes(&self) -> &[Scope] {
        &self.scopes
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
