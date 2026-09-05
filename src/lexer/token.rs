use crate::diagnostics::Span;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TType {
    ///Operators
    Plus, //+
    Minus,       //-
    Asterisk,    //*
    Slash,       // `/`
    Dot,         //.
    Pipe,        //|
    Ampersand,   //&
    Gt,          //>
    Lt,          //<
    Lbrace,      //{
    Rbrace,      //}
    Lparen,      //(
    Rparen,      //)
    Bang,        //`!`
    Colon,       //:
    Assign,      //=
    Comma,       //,
    Equality,    //==
    Neq,         //`!=`
    AddAssign,   //+=
    SubAssign,   //-=
    Gte,         //>=
    Lte,         //<=
    FatArrow,    //=>
    DoubleColon, //::
    Range,       //..
    Semicolon,   //;

    ///Primitive expression tokens
    True, //true
    False,        //false
    Identifier,   //a
    I8Literal,    //7i8
    U8Literal,    //7u8
    I16Literal,   //7i16
    U16Literal,   //7u16
    I32Literal,   //7i32
    U32Literal,   //7u32
    I64Literal,   //7i64
    U64Literal,   //8u64
    F32Literal,   //8.7f32
    F64Literal,   //8.7f64
    IntLiteral,   //8
    FloatLiteral, //8.7

    ///keywords
    Let,
    Mut,
    Func,
    Enum,
    Union,
    Record,
    Alias,
    As,
    While,
    For,
    If,
    Else,
    Match,
    Server,
    Client,
    Async,
    Await,
    Import,
    From,
    Return,
    Extern,
    Ffi,

    ///Type keywords
    I8Key,
    U8Key,
    I16Key,
    U16Key,
    I32Key,
    U32Key,
    I64Key,
    U64Key,
    F32Key,
    F64Key,
    StrKey,
    BoolKey,

    ///EOF
    End,
    ///Illegal
    Illegal,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TType,
    pub lexeme: String,
    pub span: Span,
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
