#[macro_use] mod macro_f;

use crate::common::{make_diag, make_lexer};
use rectangular::lexer::TType;
use std::rc::Rc;

lexer_ctest!(plain_int_literal, "8", IntLiteral);
lexer_ctest!(multi_int_literal, "123", IntLiteral);

#[test]
fn int_literal_with_underscore(){
    let diag = make_diag("1_000_000");
    let mut lexer = make_lexer("1_000_000", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::IntLiteral);
    assert_eq!(tokens[0].lexeme, "1000000");
    assert_eq!(tokens[2].token_type, TType::End);
}


#[test]
fn int_literal_i8_suffix(){
    let diag = make_diag("8i8");
    let mut lexer = make_lexer("8i8", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::I8Literal);
    assert_eq!(tokens[0].lexeme, "8");
    assert_eq!(tokens[2].token_type, TType::End);
}

#[test]
fn int_literal_u8_suffix(){
    let diag = make_diag("8u8");
    let mut lexer = make_lexer("8u8", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::U8Literal);
    assert_eq!(tokens[0].lexeme, "8");
    assert_eq!(tokens[2].token_type, TType::End);
}


#[test]
fn int_literal_i16_suffix(){
    let diag = make_diag("i16");
    let mut lexer = make_lexer("16i16", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::I16Literal);
    assert_eq!(tokens[0].lexeme, "16");
    assert_eq!(tokens[2].token_type, TType::End);
}

#[test]
fn int_literal_u16_suffix(){
    let diag = make_diag("u16");
    let mut lexer = make_lexer("16u16", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::U16Literal);
    assert_eq!(tokens[0].lexeme, "16");
    assert_eq!(tokens[2].token_type, TType::End);
}


#[test]
fn int_literal_i32_suffix(){
    let diag = make_diag("32i32");
    let mut lexer = make_lexer("32i32", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::I32Literal);
    assert_eq!(tokens[0].lexeme, "32");
    assert_eq!(tokens[2].token_type, TType::End);
}


#[test]
fn int_literal_u32_suffix(){
    let diag = make_diag("32u32");
    let mut lexer = make_lexer("32u32", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::U32Literal);
    assert_eq!(tokens[0].lexeme, "32");
    assert_eq!(tokens[2].token_type, TType::End);
}

#[test]
fn int_literal_u64_suffix(){
    let diag = make_diag("64u64");
    let mut lexer = make_lexer("64u64", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::U64Literal);
    assert_eq!(tokens[0].lexeme, "64");
    assert_eq!(tokens[2].token_type, TType::End);
}


#[test]
fn int_literal_i64_suffix(){
    let diag = make_diag("64i32");
    let mut lexer = make_lexer("64i64", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::I64Literal);
    assert_eq!(tokens[0].lexeme, "64");
    assert_eq!(tokens[2].token_type, TType::End);
}

lexer_ctest!(float_literal, "6.7", FloatLiteral);

#[test]
fn float_32_literal(){
    let diag = make_diag("6.7f32");
    let mut lexer = make_lexer("6.7f32", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::F32Literal);
    assert_eq!(tokens[0].lexeme, "6.7");
    assert_eq!(tokens[2].token_type, TType::End);
}

#[test]
fn float_f64_literal(){
    let diag = make_diag("6.7f64");
    let mut lexer = make_lexer("6.7f64", diag);
    let tokens = lexer.tokenize();
    println!("Tokens arry: {:?}", tokens);
    assert_eq!(tokens[0].token_type, TType::F64Literal);
    assert_eq!(tokens[0].lexeme, "6.7");
    assert_eq!(tokens[2].token_type, TType::End);
}

lexer_ctest!(plain_hex_literal, "0xff", IntLiteral);

#[test]
fn hex_literal_with_suffix(){
    let diag = make_diag("0xffu32");
    let mut lexer = make_lexer("0xffu32", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::U32Literal);
    assert_eq!(tokens[0].lexeme, "0xff");
    assert_eq!(tokens[2].token_type, TType::End);
}

#[test]
fn hex_literal_with_underscore(){
    let diag = make_diag("0xff_ff");
    let mut lexer = make_lexer("0xff_ff", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::IntLiteral);
    assert_eq!(tokens[0].lexeme, "0xffff");
    assert_eq!(tokens[2].token_type, TType::End);
}

lexer_ctest!(plain_binary_literal, "0b1010", IntLiteral);


#[test]
fn binary_literal_with_suffix(){
    let diag = make_diag("0b1010u8");
    let mut lexer = make_lexer("0b1010u8", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::U8Literal);
    assert_eq!(tokens[0].lexeme, "0b1010");
    assert_eq!(tokens[2].token_type, TType::End);
}

#[test]
fn binary_literal_with_underscore(){
    let diag = make_diag("0b1010_1010");
    let mut lexer = make_lexer("0b1010_1010", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::IntLiteral);
    assert_eq!(tokens[0].lexeme, "0b10101010");
    assert_eq!(tokens[2].token_type, TType::End);
}

#[test]
fn string_literal() {
    let diag = make_diag(r#""this is a string.""#);
    let mut lexer = make_lexer(r#""this is a string.""#, diag);
    let tokens = lexer.tokenize();

    println!("{:?}", tokens);

    assert_eq!(tokens[0].token_type, TType::StringLiteral);
    assert_eq!(tokens[0].lexeme, "this is a string.");
    assert_eq!(tokens[2].token_type, TType::End);
}

#[test]
fn char8_literal() {
    let diag = make_diag("'a'");
    let mut lexer = make_lexer("'a'", diag);
    let tokens = lexer.tokenize();

    println!("{:?}", tokens);

    assert_eq!(tokens[0].token_type, TType::Char8Literal);
    assert_eq!(tokens[2].token_type, TType::End);
}


#[test]
fn char32_literal() {
    let diag = make_diag("'🚀'c32");
    let mut lexer = make_lexer("'🚀'c32", diag);
    let tokens = lexer.tokenize();

    println!("{:?}", tokens);

    assert_eq!(tokens[0].token_type, TType::Char32Literal);
    assert_eq!(tokens[2].token_type, TType::End);
}

#[test]
fn char16_literal() {
    let diag = make_diag("'a'c16");
    let mut lexer = make_lexer("'a'c16", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Char16Literal);
    assert_eq!(tokens[2].token_type, TType::End);
}

#[test]
fn unterminated_char_literal() {
    let diag = make_diag("'a");
    let mut lexer = make_lexer("'a", Rc::clone(&diag));
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Illegal);
    assert_eq!(diag.borrow().errors.len(), 1);
}


#[test]
fn empty_char_literal() {
    let diag = make_diag("''");
    let mut lexer = make_lexer("''", Rc::clone(&diag));
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Illegal);
    assert_eq!(diag.borrow().errors.len(), 1);
}


#[test]
fn multichars_char_literal() {
    let diag = make_diag("'a b'");
    let mut lexer = make_lexer("'a b'", Rc::clone(&diag));
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Illegal);
    assert_eq!(diag.borrow().errors.len(), 1);
}

#[test]
fn unterminated_hex_literal(){
    let diag = make_diag("0x");
    let mut lexer = make_lexer("0x", Rc::clone(&diag));
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Illegal);
    assert_eq!(diag.borrow().errors.len(), 1);
}

#[test]
fn unterminated_binary_literal(){
    let diag = make_diag("0b");
    let mut lexer = make_lexer("0b", Rc::clone(&diag));
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Illegal);
    assert_eq!(diag.borrow().errors.len(), 1);
}

#[test]
fn int_literal_garbage_suffix(){
    let diag = make_diag("8xyz");
    let mut lexer = make_lexer("8xyz", Rc::clone(&diag));
    let tokens = lexer.tokenize();
    println!("Token array: {:?}", tokens);
    assert_eq!(tokens[0].token_type, TType::Illegal);
    assert_eq!(tokens[0].lexeme, "8");
    assert_eq!(tokens[1].token_type, TType::End);
    assert_eq!(diag.borrow().errors.len(), 1);
}
