mod common;

use common::{make_diag, make_lexer};
use rectangular::lexer::TType;

#[test]
fn single_line_comment(){
    let diag = make_diag("#This is a single line comment\n+");
    let mut lexer = make_lexer("#This is a single line comment\n+", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Plus);
    assert_eq!(tokens[0].lexeme, "+");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn multi_line_comment(){
    let diag = make_diag("##This is a multi_line comment##\n+");
    let mut lexer = make_lexer("##This is a  multi_line comment##\n+", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Plus);
    assert_eq!(tokens[0].lexeme, "+");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn unterminated_multi_line_comment(){
    let diag = make_diag("## This is an unterminated multi_line comment ");
    let mut lexer = make_lexer("## This is an unterminated multi_line comment ", diag.clone());
    let tokens = lexer.tokenize();

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TType::End);
    assert_eq!(diag.borrow().errors.len(), 1);
}

#[test]
fn single_line_comment_with_no_trailing_newline(){
    let diag = make_diag("# comment with no newline at all");
    let mut lexer = make_lexer("# comment with no newline at all", diag.clone());
    let tokens = lexer.tokenize();

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TType::End);
    assert_eq!(diag.borrow().errors.len(), 0);
}

#[test]
fn multi_token_sequence(){
    let diag = make_diag("1 + 2");
    let mut lexer = make_lexer("1 + 2", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].token_type, TType::IntLiteral);
    assert_eq!(tokens[0].lexeme, "1");
    assert_eq!(tokens[1].token_type, TType::Plus);
    assert_eq!(tokens[1].lexeme, "+");
    assert_eq!(tokens[2].token_type, TType::IntLiteral);
    assert_eq!(tokens[2].lexeme, "2");
    assert_eq!(tokens[3].token_type, TType::End);
}
