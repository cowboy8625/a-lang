macro_rules! snapshot {
    ($name:tt, $path:tt) => {
        #[test]
        fn $name() {
            use super::parse;
            use crate::lexer::lex;
            let contents = include_str!($path);
            let tokens = lex(contents).unwrap();
            let ast = parse(tokens).unwrap();
            let ast_string = ast
                .iter()
                .map(|node| format!("{node}\n"))
                .collect::<String>();
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_path("testdata/output/");
            settings.bind(|| {
                insta::assert_snapshot!(ast_string);
            });
        }
    };
}

snapshot!(binary, "testdata/snapshots/binary.a");
snapshot!(ifelse, "testdata/snapshots/ifelse.a");
snapshot!(max, "testdata/snapshots/max.a");
