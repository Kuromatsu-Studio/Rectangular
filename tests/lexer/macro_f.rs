#[macro_export]
macro_rules! test {
    ($fname:ident, $teststr:expr, $ttype:ident) => {
        #[test]
        fn $fname(){
            let diag = make_diag($teststr);
            let mut lexer = make_lexer($teststr, diag);
            let tokens = lexer.tokenize();

            assert_eq!(tokens[0].token_type, TType::$ttype);
            assert_eq!(tokens[0].lexeme, $teststr);
            assert_eq!(tokens[2].token_type, TType::End);
        }
    };
}
