mod common;

use common::{make_diag, make_lexer};
use rectangular::lexer::TType;

#[test]
fn plain_int_literal(){
    let diag = make_diag("8");
    let mut lexer = make_lexer("8", diag);
    let tokens =  lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::IntLiteral);
    assert_eq!(tokens[0].lexeme, "8");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn multi_int_literal(){
    let diag = make_diag("123");
    let mut lexer = make_lexer("123", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::IntLiteral);
    assert_eq!(tokens[0].lexeme, "123");
    assert_eq!(tokens[1].token_type, TType::End);
}


