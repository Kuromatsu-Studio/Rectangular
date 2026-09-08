use crate::{
    ast::{Expr, types::ASTType},
    diagnostics::Span,
};

///Paramters like a: i32
#[derive(Debug)]
pub struct ASTParam {
    pub name: Box<Expr>,
    pub ty: Box<ASTType>,
}

#[derive(Debug)]
pub enum StmtKind {
    Record {
        name: Box<Expr>,
        params: Vec<ASTParam>,
    },
    Enum {
        name: Box<Expr>,
        block: Vec<Expr>,
    },
    ///This represents a let declaration it specifically houses a let expression
    Let(Box<Expr>),
    FuncDef {
        name: Box<Expr>,
        params: Vec<ASTParam>,
        ret_ty: Box<ASTType>,
        body: Box<Expr>,
    },
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
