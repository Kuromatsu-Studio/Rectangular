use crate::diagnostics::Span;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TType {
    ///Operators
    Plus, //+
    Minus,        //-
    Asterisk,     //*
    Slash,        // `/`
    Dot,          //.
    Pipe,         //|
    Ampersand,    //&
    Gt,           //>
    Lt,           //<
    Lbrace,       //{
    Rbrace,       //}
    Lparen,       //(
    Rparen,       //)
    Lbracket,     //[
    Rbracket,     //]
    Bang,         //`!`
    Colon,        //:
    Assign,       //=
    Comma,        //,
    BindAssign,   //:=
    Equality,     //==
    Neq,          //`!=`
    AddAssign,    //+=
    SubAssign,    //-=
    DivAssign,    //`/=`
    MulAssign,    //*=
    Gte,          //>=
    Lte,          //<=
    FatArrow,     //=>
    Arrow,        //->
    DoubleColon,  //::
    Range,        //..
    Semicolon,    //;
    Dollar,       //$
    DoubleDollar, //$$

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
    Macro,
    Type,
    Return,
    Bind,
    Struct,
    If,
    Else,
    Break,
    Continue,
    While,

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
    BoolKey,

    ///EOF
    End,
    ///Illegal
    Illegal,
}

pub fn ends_expression(t: &TType) -> bool {
    matches!(
        t,
        TType::Identifier
            | TType::IntLiteral
            | TType::FloatLiteral
            | TType::I8Literal
            | TType::U8Literal
            | TType::I16Literal
            | TType::U16Literal
            | TType::I32Literal
            | TType::U32Literal
            | TType::I64Literal
            | TType::U64Literal
            | TType::F32Literal
            | TType::F64Literal
            | TType::True
            | TType::False
            | TType::I8Key
            | TType::U8Key
            | TType::I16Key
            | TType::U16Key
            | TType::I32Key
            | TType::U32Key
            | TType::I64Key
            | TType::U64Key
            | TType::F32Key
            | TType::F64Key
            | TType::BoolKey
            | TType::Rparen
            | TType::Rbracket
            | TType::Rbrace
            | TType::Return
            | TType::Continue
            | TType::Break
    )
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
