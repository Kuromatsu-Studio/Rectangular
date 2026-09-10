use crate::{
    ast::{ASTParam, ASTType, ast::BinaryOp},
    diagnostics::Span,
};

#[derive(Debug)]
pub enum DeclPatternKind {
    /// Something like x
    Name(Box<Expr>),
    ///(x,y,z)
    Tuple(Vec<DeclPattern>),
    ///Point{x,y} , Point{x:a ,y:b}(field_name, binding_name)
    Record {
        name: Box<Expr>,
        fields: Vec<(Box<Expr>, Option<Box<Expr>>)>,
    },
    /// _
    Wildcard,
}

#[derive(Debug)]
pub struct DeclPattern {
    kind: DeclPatternKind,
    span: Span,
}

impl DeclPattern {
    pub fn new(kind: DeclPatternKind, span: Span) -> Self {
        DeclPattern { kind, span }
    }
}

///The kind of expression u are dealing with
#[derive(Debug)]
pub enum ExprKind {
    Literal(ExprLiteral),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Block(Vec<Expr>),
    Let {
        pattern: DeclPattern,
        ty: Option<ASTType>,
        init: Box<Expr>,
    },
    If {
        cond: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Option<Expr>>,
    },
    While {
        cond: Box<Expr>,
        body: Box<Expr>,
    },
    Lambda {
        params: Vec<ASTParam>,
        ret_ty: Option<ASTType>,
        block: Box<Expr>,
    },
    Tuple(Vec<Expr>),
    Identifier(String),
    Return(Box<Expr>),
}

///These represent the literals like a raw number
#[derive(Debug)]
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
#[derive(Debug)]
pub struct Expr {
    kind: ExprKind,
    pub span: Span,
}

impl Expr {
    pub fn new(kind: ExprKind, span: Span) -> Self {
        Expr { kind, span }
    }
}
