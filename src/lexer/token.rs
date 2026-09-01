use crate::diagnostics::Span;

#[derive(Clone,Debug,PartialEq, Eq)]
pub enum TType {
    Plus,
    Minus,
    Asterisk, 
    Slash,
    ///Let keyword token
    Let,
    ///End of file token
    End,
}

#[derive(Clone,Debug)]
pub struct Token {
    pub token_type: TType,
    lexeme: String,
    span: Span,
}

impl Token {
    pub fn new(ttype: TType, lexeme: String, span: Span) -> Token {
        Token {
            token_type: ttype,
            lexeme,
            span,
        }
    }
}
