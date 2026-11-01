use crate::{
    ast::{
        ASTType,
        ast::{BinaryOp, UnaryOp},
    },
    diagnostics::Span,
};

#[derive(Debug)]
pub struct ASTParam {
    pub name: Box<ASTExpr>,
    pub ty: Box<ASTType>,
}

#[derive(Debug)]
pub struct ASTConstructorMember {
    pub name: Box<ASTExpr>,
    pub val: Box<ASTExpr>,
}

///The kind of expression u are dealing with
#[derive(Debug)]
pub enum ASTExprKind {
    Literal(ASTExprLiteral),
    Binary(Box<ASTExpr>, BinaryOp, Box<ASTExpr>),
    Unary(UnaryOp, Box<ASTExpr>),
    Block(Vec<ASTExpr>),
    Bind {
        mutable: bool,
        names: Vec<ASTExpr>,
        ty: Option<ASTType>,
        init: Option<Box<ASTExpr>>,
    },
    Lambda {
        params: Vec<ASTParam>,
        ret_ty: Option<ASTType>,
        body: Box<ASTExpr>,
    },
    Struct {
        members: Vec<ASTParam>,
    },
    StructConstructor {
        name: Box<ASTExpr>,
        members: Vec<ASTConstructorMember>,
    },
    IfExpr {
        condition: Box<ASTExpr>,
        if_body: Box<ASTExpr>,
        else_body: Option<Box<ASTExpr>>,
    },
    WhileExpr {
        condition: Box<ASTExpr>,
        body: Box<ASTExpr>,
    },
    Dereference(Box<ASTExpr>),
    AddressOf(Box<ASTExpr>),
    TypeExpr(Box<ASTType>),
    DollarExpr(Box<ASTExpr>),
    DoubleDollar(Box<ASTExpr>),
    Break,
    Continue,
    Return(Option<Box<ASTExpr>>),
    Call(Box<ASTExpr>, Vec<ASTExpr>),
    Index(Box<ASTExpr>, Box<ASTExpr>),
    Array(Vec<ASTExpr>),
    Identifier(String),
}

///These represent the literals like a raw number
#[derive(Debug)]
pub enum ASTExprLiteral {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
    Bool(bool),
    Str(String),
    Char8(u8),
    Char16(u16),
    Char32(char),
}

///The overall expression structure
#[derive(Debug)]
pub struct ASTExpr {
    pub kind: ASTExprKind,
    pub span: Span,
}

impl ASTExpr {
    pub fn new(kind: ASTExprKind, span: Span) -> Self {
        ASTExpr { kind, span }
    }
}
