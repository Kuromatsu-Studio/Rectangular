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
fn int_literal_i16_suffix(){
    let diag = make_diag("i16");
    let mut lexer = make_lexer("16i16", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::I16Literal);
    assert_eq!(tokens[0].lexeme, "16");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn int_literal_u16_suffix(){
    let diag = make_diag("u16");
    let mut lexer = make_lexer("16u16", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::U16Literal);
    assert_eq!(tokens[0].lexeme, "16");
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

#[test]
fn int_literal_i64_suffix(){
    let diag = make_diag("67i64");
    let mut lexer = make_lexer("67i64", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::I64Literal);
    assert_eq!(tokens[0].lexeme, "67");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn int_literal_u64_suffix(){
    let diag = make_diag("67u64");
    let mut lexer = make_lexer("67u64", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::U64Literal);
    assert_eq!(tokens[0].lexeme, "67");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn float_literal(){
    let diag = make_diag("6.7");
    let mut lexer = make_lexer("6.7", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::FloatLiteral);
    assert_eq!(tokens[0].lexeme, "6.7");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn float_f32_literal(){
    let diag = make_diag("6.7f32");
    let mut lexer = make_lexer("6.7f32", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::F32Literal);
    assert_eq!(tokens[0].lexeme, "6.7");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn float_f64_literal(){
    let diag = make_diag("6.7f64");
    let mut lexer = make_lexer("6.7f64", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::F64Literal);
    assert_eq!(tokens[0].lexeme, "6.7");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn plain_hex_literal(){
    let diag = make_diag("0xff");
    let mut lexer = make_lexer("0xff", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::IntLiteral);
    assert_eq!(tokens[0].lexeme, "0xff");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn hex_literal_with_suffix(){
    let diag = make_diag("0xffu32");
    let mut lexer = make_lexer("0xffu32", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::U32Literal);
    assert_eq!(tokens[0].lexeme, "0xff");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn hex_literal_with_underscore(){
    let diag = make_diag("0xff_ff");
    let mut lexer = make_lexer("0xff_ff", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::IntLiteral);
    assert_eq!(tokens[0].lexeme, "0xffff");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn plain_binary_literal(){
    let diag = make_diag("0b1010");
    let mut lexer = make_lexer("0b1010", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::IntLiteral);
    assert_eq!(tokens[0].lexeme, "0b1010");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn binary_literal_with_suffix(){
    let diag = make_diag("0b1010u8");
    let mut lexer = make_lexer("0b1010u8", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::U8Literal);
    assert_eq!(tokens[0].lexeme, "0b1010");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn binary_literal_with_underscore(){
    let diag = make_diag("0b1010_1010");
    let mut lexer = make_lexer("0b1010_1010", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::IntLiteral);
    assert_eq!(tokens[0].lexeme, "0b10101010");
    assert_eq!(tokens[1].token_type, TType::End);
}
