use crate::{diagnostics::Span, hir::expr::HIRParam};

pub struct ExprId(usize);

///This is an id to the actual type node like the node i32
pub struct TypeNodeId(usize);

///This is an id to the type like all i32's getting a type_id of 0
pub struct TypeId(usize);

pub struct AttrId(usize);

pub struct HIRModule {}
