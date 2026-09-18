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
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn identifier_starting_with_keyword(){
    let diag = make_diag("lett");
    let mut lexer = make_lexer("lett", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Identifier);
    assert_eq!(tokens[0].lexeme, "lett");
    assert_eq!(tokens[1].token_type, TType::End);
}

// Keywords

#[test]
fn keyword_let(){
    let diag = make_diag("let");
    let mut lexer = make_lexer("let", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::Let);
    assert_eq!(tokens[0].lexeme, "let");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_mut(){
    let diag = make_diag("mut");
    let mut lexer = make_lexer("mut", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::Mut);
    assert_eq!(tokens[0].lexeme, "mut");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_func(){
    let diag = make_diag("func");
    let mut lexer = make_lexer("func", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::Func);
    assert_eq!(tokens[0].lexeme, "func");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_enum(){
    let diag = make_diag("enum");
    let mut lexer = make_lexer("enum", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::Enum);
    assert_eq!(tokens[0].lexeme, "enum");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_union(){
    let diag = make_diag("union");
    let mut lexer = make_lexer("union", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::Union);
    assert_eq!(tokens[0].lexeme, "union");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_record(){
    let diag = make_diag("record");
    let mut lexer = make_lexer("record", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::Record);
    assert_eq!(tokens[0].lexeme, "record");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_alias(){
    let diag = make_diag("alias");
    let mut lexer = make_lexer("alias", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::Alias);
    assert_eq!(tokens[0].lexeme, "alias");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_as(){
    let diag = make_diag("as");
    let mut lexer = make_lexer("as", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::As);
    assert_eq!(tokens[0].lexeme, "as");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_while(){
    let diag = make_diag("while");
    let mut lexer = make_lexer("while", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::While);
    assert_eq!(tokens[0].lexeme, "while");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_for(){
    let diag = make_diag("for");
    let mut lexer = make_lexer("for", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::For);
    assert_eq!(tokens[0].lexeme, "for");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_if(){
    let diag = make_diag("if");
    let mut lexer = make_lexer("if", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::If);
    assert_eq!(tokens[0].lexeme, "if");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_else(){
    let diag = make_diag("else");
    let mut lexer = make_lexer("else", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::Else);
    assert_eq!(tokens[0].lexeme, "else");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_match(){
    let diag = make_diag("match");
    let mut lexer = make_lexer("match", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::Match);
    assert_eq!(tokens[0].lexeme, "match");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_server(){
    let diag = make_diag("server");
    let mut lexer = make_lexer("server", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::Server);
    assert_eq!(tokens[0].lexeme, "server");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_client(){
    let diag = make_diag("client");
    let mut lexer = make_lexer("client", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::Client);
    assert_eq!(tokens[0].lexeme, "client");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_async(){
    let diag = make_diag("async");
    let mut lexer = make_lexer("async", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::Async);
    assert_eq!(tokens[0].lexeme, "async");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_await(){
    let diag = make_diag("await");
    let mut lexer = make_lexer("await", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::Await);
    assert_eq!(tokens[0].lexeme, "await");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_import(){
    let diag = make_diag("import");
    let mut lexer = make_lexer("import", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::Import);
    assert_eq!(tokens[0].lexeme, "import");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_from(){
    let diag = make_diag("from");
    let mut lexer = make_lexer("from", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::From);
    assert_eq!(tokens[0].lexeme, "from");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_return(){
    let diag = make_diag("return");
    let mut lexer = make_lexer("return", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::Return);
    assert_eq!(tokens[0].lexeme, "return");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_extern(){
    let diag = make_diag("extern");
    let mut lexer = make_lexer("extern", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::Extern);
    assert_eq!(tokens[0].lexeme, "extern");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_ffi(){
    let diag = make_diag("ffi");
    let mut lexer = make_lexer("ffi", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::Ffi);
    assert_eq!(tokens[0].lexeme, "ffi");
    assert_eq!(tokens[1].token_type, TType::End);
}

// True / False

#[test]
fn keyword_true(){
    let diag = make_diag("true");
    let mut lexer = make_lexer("true", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::True);
    assert_eq!(tokens[0].lexeme, "true");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_false(){
    let diag = make_diag("false");
    let mut lexer = make_lexer("false", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::False);
    assert_eq!(tokens[0].lexeme, "false");
    assert_eq!(tokens[1].token_type, TType::End);
}

// Type keywords

#[test]
fn keyword_i8_type(){
    let diag = make_diag("i8");
    let mut lexer = make_lexer("i8", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::I8Key);
    assert_eq!(tokens[0].lexeme, "i8");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_u8_type(){
    let diag = make_diag("u8");
    let mut lexer = make_lexer("u8", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::U8Key);
    assert_eq!(tokens[0].lexeme, "u8");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_i16_type(){
    let diag = make_diag("i16");
    let mut lexer = make_lexer("i16", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::I16Key);
    assert_eq!(tokens[0].lexeme, "i16");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_u16_type(){
    let diag = make_diag("u16");
    let mut lexer = make_lexer("u16", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::U16Key);
    assert_eq!(tokens[0].lexeme, "u16");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_i32_type(){
    let diag = make_diag("i32");
    let mut lexer = make_lexer("i32", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::I32Key);
    assert_eq!(tokens[0].lexeme, "i32");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_u32_type(){
    let diag = make_diag("u32");
    let mut lexer = make_lexer("u32", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::U32Key);
    assert_eq!(tokens[0].lexeme, "u32");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_i64_type(){
    let diag = make_diag("i64");
    let mut lexer = make_lexer("i64", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::I64Key);
    assert_eq!(tokens[0].lexeme, "i64");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_u64_type(){
    let diag = make_diag("u64");
    let mut lexer = make_lexer("u64", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::U64Key);
    assert_eq!(tokens[0].lexeme, "u64");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_f32_type(){
    let diag = make_diag("f32");
    let mut lexer = make_lexer("f32", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::F32Key);
    assert_eq!(tokens[0].lexeme, "f32");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_f64_type(){
    let diag = make_diag("f64");
    let mut lexer = make_lexer("f64", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::F64Key);
    assert_eq!(tokens[0].lexeme, "f64");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_str_type(){
    let diag = make_diag("str");
    let mut lexer = make_lexer("str", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::StrKey);
    assert_eq!(tokens[0].lexeme, "str");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn keyword_bool_type(){
    let diag = make_diag("bool");
    let mut lexer = make_lexer("bool", diag);
    let tokens = lexer.tokenize();
    assert_eq!(tokens[0].token_type, TType::BoolKey);
    assert_eq!(tokens[0].lexeme, "bool");
    assert_eq!(tokens[1].token_type, TType::End);
}
