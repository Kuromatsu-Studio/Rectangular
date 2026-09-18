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

