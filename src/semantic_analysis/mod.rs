mod analysis;
#[cfg(test)]
mod test;
pub use analysis::SemanticAnalysisVisitor;
pub use analysis::SymbolTableBuilder;

use crate::lexer::Span;
use crate::parse::ast;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Unit,
    U64,
}

impl Default for Type {
    fn default() -> Self {
        Self::Unit
    }
}

impl From<&ast::Type> for Type {
    fn from(ast::Type(ast::Ident { value, .. }): &ast::Type) -> Self {
        match value.as_str() {
            "()" => Self::Unit,
            "u64" => Self::U64,
            _ => unimplemented!("{value:?}"),
        }
    }
}

impl Type {
    pub fn _size(&self) -> usize {
        match self {
            Self::Unit => 0,
            Self::U64 => 8,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "()"),
            Self::U64 => write!(f, "u64"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    Global,
    Block,
    Param,
}

impl Default for Scope {
    fn default() -> Self {
        Self::Global
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Symbol {
    Variable(Variable),
    Function(Function),
}

macro_rules! into_symbol {
    ($name:ident) => {
        impl From<$name> for Symbol {
            fn from(value: $name) -> Self {
                Self::$name(value)
            }
        }
    };
}

into_symbol!(Variable);
into_symbol!(Function);

impl Symbol {
    pub fn name(&self) -> &str {
        match self {
            Self::Variable(v) => &v.name,
            Self::Function(f) => &f.name,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    pub scope: Scope,
    pub name: String,
    pub ty: Type,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub scope: Scope,
    pub name: String,
    pub params: Vec<Variable>,
    pub ret: Type,
    pub span: Span,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SymbolTable {
    pub symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn insert(&mut self, symbol: Symbol) {
        self.symbols.insert(symbol.name().to_string(), symbol);
    }

    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    pub fn _delete(&mut self, name: &str) {
        self.symbols.remove(name);
    }
}
