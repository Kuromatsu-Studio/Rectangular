use crate::hir::hir::AttrId;

#[derive(Debug)]
pub struct HIRAttr {
    id: AttrId,
    name: String,
}
