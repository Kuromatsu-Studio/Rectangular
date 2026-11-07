#[macro_use] mod macro_f;

use crate::common::{make_diag, make_lexer};
use rectangular::lexer::{self, TType};

// common_test(function name, string of what you want to test, the type you expect)
lexer_ctest!(plain_identifier, "myvar", Identifier);
lexer_ctest!(identifier_starting_with_keyword, "lett", Identifier);

// Keywords
lexer_ctest!(keyword_true, "true", True);
lexer_ctest!(keyword_false, "false", False);
lexer_ctest!(keyword_return, "return",  Return);
lexer_ctest!(keyword_continue, "continue", Continue);
lexer_ctest!(keyword_break, "break", Break);

#[test]
fn keyword_struct() {
    let diag = make_diag("struct");
    let mut lexer = make_lexer("struct", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Struct);
    assert_eq!(tokens[0].lexeme, "struct");
    assert_eq!(tokens[1].token_type, TType::End)
}

#[test]
fn keyword_bind() {
    let diag = make_diag("bind");
    let mut lexer = make_lexer("bind", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Bind);
    assert_eq!(tokens[0].lexeme, "bind");
    assert_eq!(tokens[1].token_type, TType::End)
}

#[test]
fn keyword_if() {
    let diag = make_diag("if");
    let mut lexer = make_lexer("if", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::If);
    assert_eq!(tokens[0].lexeme, "if");
    assert_eq!(tokens[1].token_type, TType::End)
}

#[test]
fn keyword_else() {
    let diag = make_diag("else");
    let mut lexer = make_lexer("else", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Else);
    assert_eq!(tokens[0].lexeme, "else");
    assert_eq!(tokens[1].token_type, TType::End)
}

// Type keywords
lexer_ctest!(keyword_i8_type, "i8", I8Key);
lexer_ctest!(keyword_u8_type, "u8", U8Key);
lexer_ctest!(keyword_i16_type, "i16", I16Key);
lexer_ctest!(keyword_u16_type, "u16", U16Key);
lexer_ctest!(keyword_i32_type, "i32", I32Key);
lexer_ctest!(keyword_u32_type, "u32", U32Key);
lexer_ctest!(keyword_i64_type, "i64", I64Key);
lexer_ctest!(keyword_u64_type, "u64", U64Key);
lexer_ctest!(keyword_f32_type, "f32", F32Key);
lexer_ctest!(keyword_f64_type, "f64", F64Key);
lexer_ctest!(keyword_bool_type, "bool", BoolKey);
lexer_ctest!(keyword_typekey, "type", TypeKey);

#[test]
fn keyword_U0key() {
    let diag = make_diag("u0");
    let mut lexer = make_lexer("u0", diag);
    let tokens = lexer.tokenize();
    println!("{:?}", tokens);
    assert_eq!(tokens[0].token_type, TType::U0Key);
    assert_eq!(tokens[0].lexeme, "u0");
    assert_eq!(tokens[2].token_type, TType::End)
}

#[test]
fn keyword_c8key() {
    let diag = make_diag("c8");
    let mut lexer = make_lexer("c8", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::C8Key);
    assert_eq!(tokens[0].lexeme, "c8");
    assert_eq!(tokens[1].token_type, TType::End)
}

#[test]
fn keyword_c16key() {
    let diag = make_diag("c16");
    let mut lexer = make_lexer("c16", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::C16Key);
    assert_eq!(tokens[0].lexeme, "c16");
    assert_eq!(tokens[1].token_type, TType::End)
}

#[test]
fn keyword_c32key() {
    let diag = make_diag("c32");
    let mut lexer = make_lexer("c32", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::C32Key);
    assert_eq!(tokens[0].lexeme, "c32");
    assert_eq!(tokens[1].token_type, TType::End)
}

#[test]
fn keyword_fn() {
    let diag = make_diag("fn");
    let mut lexer = make_lexer("fn", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Fn);
    assert_eq!(tokens[0].lexeme, "fn");
    assert_eq!(tokens[1].token_type, TType::End)
}
