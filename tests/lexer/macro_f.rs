macro_rules! lexer_ctest {
    ($fname:ident, $teststr:expr, $ttype:ident) => {
        #[test]
        fn $fname(){
            let diag = make_diag($teststr);
            let mut lexer = make_lexer($teststr, diag);
            let tokens = lexer.tokenize();
            println!("{:?}", tokens);
            assert_eq!(tokens[0].token_type, TType::$ttype);
            assert_eq!(tokens[0].lexeme, $teststr);
            assert_eq!(tokens[2].token_type, TType::End);
        }
    };
}

macro_rules! lexer_otest {
    ($fname:ident, $teststr:expr, $ttype:ident) => {
        #[test]
        fn $fname(){
            let diag = make_diag($teststr);
            let mut lexer = make_lexer($teststr, diag);
            let tokens = lexer.tokenize();

            assert_eq!(tokens[0].token_type, TType::$ttype);
            assert_eq!(tokens[0].lexeme, $teststr);
            assert_eq!(tokens[1].token_type, TType::End);
        }
    };
}
