use super::{Scope, SemanticAnalysisVisitor, Symbol, SymbolTableBuilder, Type};
use crate::lexer::lex;
use crate::parse::parse;
#[test]
fn symbol_table_test() {
    let tokens = lex("
    fn add(a: u64, b: u64) -> u64 {
        return a + b;
    }
    fn main() {
        return add(1,2);
    }
    ")
    .unwrap();
    let ast = parse(tokens).unwrap();
    let mut builder = SymbolTableBuilder::default();
    builder.visit(&ast);
    let symbol_table = builder.build();
    let Symbol::Variable(var_param_a) = symbol_table.get("a").unwrap() else {
        unreachable!()
    };
    assert_eq!(var_param_a.scope, Scope::Param);
    assert_eq!(var_param_a.name, "a");
    assert_eq!(var_param_a.ty, Type::U64);
}
