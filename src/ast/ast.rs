use crate::{ast::operators::BinaryOp, diagnostics::Span};

///The kind of expression u are dealing with
pub enum ExprKind {
    Literal(ExprLiteral),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Block(Vec<Expr>),
    While { cond: Box<Expr>, body: Box<Expr> },
    Return(Box<Expr>),
}

///These represent the literals like a raw number
pub enum ExprLiteral {
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
}

///The overall expression structure
pub struct Expr {
    kind: ExprKind,
    span: Span,
}

impl Expr {
    pub fn new(kind: ExprKind, span: Span) -> Self {
        Expr { kind, span }
    }
}
