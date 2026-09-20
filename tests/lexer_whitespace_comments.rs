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
