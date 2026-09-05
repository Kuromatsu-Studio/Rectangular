use crate::{
    ast::{Expr, types::ASTType},
    diagnostics::Span,
};



///Paramters like a: i32
#[derive(Debug)]
pub struct Param {
    name: Box<Expr>,
    ty: Box<ASTType>,
}

#[derive(Debug)]
pub enum StmtKind {
    Struct { name: Box<Expr>, params: Vec<Param> },
    Enum { name: Box<Expr>, block: Vec<Expr> },
}

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

impl Stmt {
    pub fn new(kind: StmtKind, span: Span) -> Self {
        Stmt { kind, span }
    }
}
