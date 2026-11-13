use crate::hir::{attrs::HIRAttr, expr::HIRExpr, types::HIRType};

#[derive(Debug, Clone, Copy)]
pub struct ExprId(pub usize);

///This is an id to the actual type node like the node i32
#[derive(Debug, Clone, Copy)]
pub struct TypeNodeId(pub usize);

///This is an id to the type like all i32's getting a type_id of 0
#[derive(Debug, Clone, Copy)]
pub struct TypeId(usize);

#[derive(Debug, Clone, Copy)]
pub struct AttrId(usize);

///This represents the entire module
pub struct HIRModule {
    pub exprs: Vec<HIRExpr>,
    pub types: Vec<HIRType>,
    pub attrs: Vec<HIRAttr>,
    pub roots: Vec<ExprId>,
}

impl HIRModule {
    pub fn new() -> HIRModule {
        HIRModule {
            exprs: Vec::new(),
            types: Vec::new(),
            attrs: Vec::new(),
            roots: Vec::new(),
        }
    }
}
