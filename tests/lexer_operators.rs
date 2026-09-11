mod common;

use common::{make_diag, make_lexer};
use rectangular::lexer::{TType, Token};


// Lexer Operator Tests.
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

#[test]
fn sub_assign(){
    let diag = make_diag("-=");
    let mut lexer = make_lexer("-=", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::SubAssign);
    assert_eq!(tokens[0].lexeme, "-=");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn asterist(){
    let diag = make_diag("*");
    let mut lexer = make_lexer("*", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Asterisk);
    assert_eq!(tokens[0].lexeme, "*");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn mul_assign(){
    let diag = make_diag("*=");
    let mut lexer = make_lexer("*=", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::MulAssign);
    assert_eq!(tokens[0].lexeme, "*=");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn div(){
    let diag = make_diag("/");
    let mut lexer = make_lexer("/", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Slash);
    assert_eq!(tokens[0].lexeme, "/");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn div_assign(){
    let diag = make_diag("/=");
    let mut lexer = make_lexer("/=", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::DivAssign);
    assert_eq!(tokens[0].lexeme, "/=");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn gt(){
    let diag = make_diag(">");
    let mut lexer = make_lexer(">", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Gt);
    assert_eq!(tokens[0].lexeme, ">");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn gte(){
    let diag = make_diag(">=");
    let mut lexer = make_lexer(">=", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Gte);
    assert_eq!(tokens[0].lexeme, ">=");
    assert_eq!(tokens[1].token_type, TType::End);
}
