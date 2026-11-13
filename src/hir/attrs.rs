use crate::{
    diagnostics::Span,
    hir::hir::{AttrId, ExprId},
};

#[derive(Debug)]
pub struct HIRAttr {
    id: AttrId,
    path: Vec<String>,
    args: Vec<ExprId>,
    span: Span,
}

impl HIRAttr {
    pub fn new(id: AttrId, path: Vec<String>, args: Vec<ExprId>, span: Span) -> HIRAttr {
        HIRAttr {
            id,
            path,
            args,
            span,
        }
    }

    pub fn get_id(&self) -> AttrId {
        self.id
    }
}
