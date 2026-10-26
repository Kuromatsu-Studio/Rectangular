mod common;

use common::{make_diag, make_lexer};
use rectangular::lexer::TType;

#[test]
fn plain_identifier(){
    let diag = make_diag("myvar");
    let mut lexer = make_lexer("myvar", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Identifier);
    assert_eq!(tokens[0].lexeme, "myvar");
    assert_eq!(tokens[2].token_type, TType::End);
}

#[test]
fn identifier_starting_with_keyword(){
    let diag = make_diag("lett");
    let mut lexer = make_lexer("lett", diag);
    let tokens = lexer.tokenize();
    println!("Tokens array: {:?}",tokens);
    assert_eq!(tokens[0].token_type, TType::Identifier);
    assert_eq!(tokens[0].lexeme, "lett");
    assert_eq!(tokens[2].token_type, TType::End);
}

// Keywords

#[test]
fn keyword_true(){
    let diag = make_diag("true");
    let mut lexer = make_lexer("true", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::True);
    assert_eq!(tokens[0].lexeme, "true");
    assert_eq!(tokens[2].token_type, TType::End);
}

#[test]
fn keyword_false(){
    let diag = make_diag("false");
    let mut lexer = make_lexer("false", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::False);
    assert_eq!(tokens[0].lexeme, "false");
    assert_eq!(tokens[2].token_type, TType::End);
}

// Type keywords
#[test]
fn keyword_i8_type(){
    let diag = make_diag("i8");
    let mut lexer = make_lexer("i8", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::I8Key);
    assert_eq!(tokens[0].lexeme, "i8");
    assert_eq!(tokens[2].token_type, TType::End);
}

#[test]
fn keyword_u8_type(){
    let diag = make_diag("u8");
    let mut lexer = make_lexer("u8", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::U8Key);
    assert_eq!(tokens[0].lexeme, "u8");
    assert_eq!(tokens[2].token_type, TType::End);
}

#[test]
fn keyword_i16_type(){
    let diag = make_diag("i16");
    let mut lexer = make_lexer("i16", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::I16Key);
    assert_eq!(tokens[0].lexeme, "i16");
    assert_eq!(tokens[2].token_type, TType::End);
}

#[test]
fn keyword_u16_type(){
    let diag = make_diag("u16");
    let mut lexer = make_lexer("u16", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::U16Key);
    assert_eq!(tokens[0].lexeme, "u16");
    assert_eq!(tokens[2].token_type, TType::End);
}

#[test]
fn keyword_i32_type(){
    let diag = make_diag("i32");
    let mut lexer = make_lexer("i32", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::I32Key);
    assert_eq!(tokens[0].lexeme, "i32");
    assert_eq!(tokens[2].token_type, TType::End);
}

#[test]
fn keyword_u32_type(){
    let diag = make_diag("u32");
    let mut lexer = make_lexer("u32", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::U32Key);
    assert_eq!(tokens[0].lexeme, "u32");
    assert_eq!(tokens[2].token_type, TType::End);
}

#[test]
fn keyword_i64_type(){
    let diag = make_diag("i64");
    let mut lexer = make_lexer("i64", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::I64Key);
    assert_eq!(tokens[0].lexeme, "i64");
    assert_eq!(tokens[2].token_type, TType::End);
}

#[test]
fn keyword_u64_type(){
    let diag = make_diag("u64");
    let mut lexer = make_lexer("u64", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::U64Key);
    assert_eq!(tokens[0].lexeme, "u64");
    assert_eq!(tokens[2].token_type, TType::End);
}

#[test]
fn keyword_f32_type(){
    let diag = make_diag("f32");
    let mut lexer = make_lexer("f32", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::F32Key);
    assert_eq!(tokens[0].lexeme, "f32");
    assert_eq!(tokens[2].token_type, TType::End);
}

#[test]
fn keyword_f64_type(){
    let diag = make_diag("f64");
    let mut lexer = make_lexer("f64", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::F64Key);
    assert_eq!(tokens[0].lexeme, "f64");
    assert_eq!(tokens[2].token_type, TType::End);
}

#[test]
fn keyword_bool_type(){
    let diag = make_diag("bool");
    let mut lexer = make_lexer("bool", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::BoolKey);
    assert_eq!(tokens[0].lexeme, "bool");
    assert_eq!(tokens[2].token_type, TType::End);
}
