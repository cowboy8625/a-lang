pub mod ast;
mod parser;
#[cfg(test)]
mod test;

pub use ast::keyword;
pub use ast::*;

use parser::Parser;

use crate::lexer::TokenStream;
use crate::symbol_table::SymbolTable;

pub fn parse(stream: TokenStream) -> Result<(Vec<Item>, SymbolTable), Vec<String>> {
    Parser::new(stream).parse()
}
