macro_rules! snapshot {
    ($name:tt, $path:tt) => {
        #[test]
        fn $name() {
            use super::parse;
            use crate::lexer::lex;
            let contents = include_str!($path);
            let tokens = lex(contents).unwrap();
            let ast = parse(tokens).unwrap();
            let ast_string = ast.0.iter().map(ToString::to_string).collect::<String>();
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_path("testdata/output/");
            settings.bind(|| {
                insta::assert_snapshot!(ast_string);
            });
        }
    };
}

snapshot!(binary, "../../snapshots/binary.a");
snapshot!(ifelse, "../../snapshots/ifelse.a");
snapshot!(max, "../../snapshots/max.a");
