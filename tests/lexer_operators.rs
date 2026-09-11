mod common;

use common::{make_diag, make_lexer};
use rectangular::lexer::TType;

#[test]
fn single_plus_token(){
    let diag = make_diag("+");
    let mut lexer =  make_lexer("+", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Plus);
    assert_eq!(tokens[0].lexeme, "+");
    assert_eq!(tokens[1].token_type, TType::End);
}
 
#[test]
fn add_assign(){
    let diag = make_diag("+=");
    let mut lexer = make_lexer("+=", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::AddAssign);
    assert_eq!(tokens[0].lexeme, "+=");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn minus(){
    let diag = make_diag("-");
    let mut lexer = make_lexer("-", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Minus);
    assert_eq!(tokens[0].lexeme, "-");
    assert_eq!(tokens[1].token_type, TType::End);
}
