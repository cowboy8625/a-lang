// use crate::semantic_analysis::{SemanticAnalysisVisitor, SymbolTableBuilder};
macro_rules! snapshot {
    ($name:tt, $path:tt) => {
        #[test]
        fn $name() {
            use super::*;
            use $crate::lexer::lex;
            use $crate::parse::parse;
            let contents = include_str!($path);
            let tokens = lex(contents).unwrap();
            let ast = parse(tokens).unwrap();
            // let mut symbol_table_builder = SymbolTableBuilder::default();
            // symbol_table_builder.visit(&ast);
            // let symbol_table = symbol_table_builder.build();
            let ir_code = code_gen(ast).unwrap();
            let result = ir_code
                .iter()
                .map(ToString::to_string)
                // .map(|i| format!("{i:#?}\n"))
                .collect::<String>();
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_path("testdata/output/");
            settings.bind(|| {
                insta::assert_snapshot!(result);
            });
        }
    };
}

snapshot!(binary, "../../snapshots/binary.a");
snapshot!(ifelse, "../../snapshots/ifelse.a");
snapshot!(max, "../../snapshots/max.a");
