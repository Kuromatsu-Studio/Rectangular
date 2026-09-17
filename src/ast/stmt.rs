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

#[derive(Debug,Clone)]
pub enum FuncTarget {
    Server,
    Client,
    None,
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
        target: FuncTarget,
        name: Box<Expr>,
        params: Vec<ASTParam>,
        ret_ty: Box<Option<ASTType>>,
        body: Box<Expr>,
    },
    TargetBlock {
        target: FuncTarget,
        funcs: Vec<ASTStmt>,
    },
}

#[derive(Debug)]
pub struct ASTStmt {
    pub kind: StmtKind,
    pub span: Span,
}

impl ASTStmt {
    pub fn new(kind: StmtKind, span: Span) -> Self {
        ASTStmt { kind, span }
    }
}
