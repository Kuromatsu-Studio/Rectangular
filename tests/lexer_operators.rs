mod common;

use common::{make_diag, make_lexer};
use rectangular::lexer::TType;

// Lexer Operator Tests.
#[test]
fn single_plus_token(){
    let diag = make_diag("+");
    let mut lexer =  make_lexer("+", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Plus);
    assert_eq!(tokens[0].lexeme, "+");
    assert_eq!(tokens[1].token_type, TType::End);
}
 
#[test]
fn add_assign(){
    let diag = make_diag("+=");
    let mut lexer = make_lexer("+=", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::AddAssign);
    assert_eq!(tokens[0].lexeme, "+=");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn minus(){
    let diag = make_diag("-");
    let mut lexer = make_lexer("-", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Minus);
    assert_eq!(tokens[0].lexeme, "-");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn sub_assign(){
    let diag = make_diag("-=");
    let mut lexer = make_lexer("-=", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::SubAssign);
    assert_eq!(tokens[0].lexeme, "-=");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn asterist(){
    let diag = make_diag("*");
    let mut lexer = make_lexer("*", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Asterisk);
    assert_eq!(tokens[0].lexeme, "*");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn mul_assign(){
    let diag = make_diag("*=");
    let mut lexer = make_lexer("*=", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::MulAssign);
    assert_eq!(tokens[0].lexeme, "*=");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn div(){
    let diag = make_diag("/");
    let mut lexer = make_lexer("/", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Slash);
    assert_eq!(tokens[0].lexeme, "/");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn div_assign(){
    let diag = make_diag("/=");
    let mut lexer = make_lexer("/=", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::DivAssign);
    assert_eq!(tokens[0].lexeme, "/=");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn gt(){
    let diag = make_diag(">");
    let mut lexer = make_lexer(">", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Gt);
    assert_eq!(tokens[0].lexeme, ">");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn gte(){
    let diag = make_diag(">=");
    let mut lexer = make_lexer(">=", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Gte);
    assert_eq!(tokens[0].lexeme, ">=");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn lt(){
    let diag = make_diag("<");
    let mut lexer = make_lexer("<", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Lt);
    assert_eq!(tokens[0].lexeme, "<");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn lte(){
    let diag = make_diag("<=");
    let mut lexer = make_lexer("<=", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Lte);
    assert_eq!(tokens[0].lexeme, "<=");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn dot(){
    let diag = make_diag(".");
    let mut lexer = make_lexer(".", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Dot);
    assert_eq!(tokens[0].lexeme, ".");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn range(){
    let diag = make_diag("..");
    let mut lexer = make_lexer("..", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Range);
    assert_eq!(tokens[0].lexeme, "..");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn pipe(){
    let diag = make_diag("|");
    let mut lexer = make_lexer("|", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Pipe);
    assert_eq!(tokens[0].lexeme, "|");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn ampersand(){
    let diag = make_diag("&");
    let mut lexer = make_lexer("&", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Ampersand);
    assert_eq!(tokens[0].lexeme, "&");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn lbrace(){
    let diag = make_diag("{");
    let mut lexer = make_lexer("{", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Lbrace);
    assert_eq!(tokens[0].lexeme, "{");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn rbrace(){
    let diag = make_diag("}");
    let mut lexer = make_lexer("}", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Rbrace);
    assert_eq!(tokens[0].lexeme, "}");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn lparen(){
    let diag = make_diag("(");
    let mut lexer = make_lexer("(", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Lparen);
    assert_eq!(tokens[0].lexeme, "(");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn rparen(){
    let diag = make_diag(")");
    let mut lexer = make_lexer(")", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Rparen);
    assert_eq!(tokens[0].lexeme, ")");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn lbracket(){
    let diag = make_diag("[");
    let mut lexer = make_lexer("[", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Lbracket);
    assert_eq!(tokens[0].lexeme, "[");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn rbracket(){
    let diag = make_diag("]");
    let mut lexer = make_lexer("]", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Rbracket);
    assert_eq!(tokens[0].lexeme, "]");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn bang(){
    let diag = make_diag("!");
    let mut lexer = make_lexer("!", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Bang);
    assert_eq!(tokens[0].lexeme, "!");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn neq(){
    let diag = make_diag("!=");
    let mut lexer = make_lexer("!=", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Neq);
    assert_eq!(tokens[0].lexeme, "!=");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn colon(){
    let diag = make_diag(":");
    let mut lexer = make_lexer(":", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Colon);
    assert_eq!(tokens[0].lexeme, ":");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn double_colon(){
    let diag = make_diag("::");
    let mut lexer = make_lexer("::", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::DoubleColon);
    assert_eq!(tokens[0].lexeme, "::");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn assign(){
    let diag = make_diag("=");
    let mut lexer = make_lexer("=", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Assign);
    assert_eq!(tokens[0].lexeme, "=");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn equality(){
    let diag = make_diag("==");
    let mut lexer = make_lexer("==", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Equality);
    assert_eq!(tokens[0].lexeme, "==");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn fat_arrow(){
    let diag = make_diag("=>");
    let mut lexer = make_lexer("=>", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::FatArrow);
    assert_eq!(tokens[0].lexeme, "=>");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn comma(){
    let diag = make_diag(",");
    let mut lexer = make_lexer(",", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Comma);
    assert_eq!(tokens[0].lexeme, ",");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn semicolon(){
    let diag = make_diag(";");
    let mut lexer = make_lexer(";", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Semicolon);
    assert_eq!(tokens[0].lexeme, ";");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn double_slash(){
    let diag = make_diag("//");
    let mut lexer = make_lexer("//", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::DoubleSlash);
    assert_eq!(tokens[0].lexeme, "//");
    assert_eq!(tokens[1].token_type, TType::End);
}

#[test]
fn bind(){
    let diag = make_diag(":=");
    let mut lexer = make_lexer(":=", diag);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_type, TType::Bind);
    assert_eq!(tokens[0].lexeme, ":=");
    assert_eq!(tokens[1].token_type, TType::End);
}
