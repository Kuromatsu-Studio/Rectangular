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

#[test]
fn int_literal_with_underscore(){
    let diag = make_diag("1_000_000");
    let mut lexer = make_lexer("1_000_000", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::IntLiteral);
    assert_eq!(tokens[0].lexeme, "1000000");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn int_literal_i8_suffix(){
    let diag = make_diag("8i8");
    let mut lexer = make_lexer("8i8", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::I8Literal);
    assert_eq!(tokens[0].lexeme, "8");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn int_literal_u8_suffix(){
    let diag = make_diag("8u8");
    let mut lexer = make_lexer("8u8", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::U8Literal);
    assert_eq!(tokens[0].lexeme, "8");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn int_literal_i32_suffix(){
    let diag = make_diag("32i32");
    let mut lexer = make_lexer("32i32", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::I32Literal);
    assert_eq!(tokens[0].lexeme, "32");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn int_literal_u32_suffix(){
    let diag = make_diag("32u32");
    let mut lexer = make_lexer("32u32", diag);
    let tokens = lexer.tokenize();
    
    assert_eq!(tokens[0].token_type, TType::U32Literal);
    assert_eq!(tokens[0].lexeme, "32");
    assert_eq!(tokens[1].token_type, TType::End);
}
