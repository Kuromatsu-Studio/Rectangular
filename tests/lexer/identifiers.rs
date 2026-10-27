#[macro_use] mod macro_f;

use crate::common::{make_diag, make_lexer};
use rectangular::lexer::TType;


// common_test(function name, string of what you want to test, the type you expect)
test!(plain_identifier, "myvar", Identifier);

test!(identifier_starting_with_keyword, "lett", Identifier);

// Keywords
test!(keyword_true, "true", True);
test!(keyword_false, "false", False);

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
