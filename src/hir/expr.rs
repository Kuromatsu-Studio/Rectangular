use crate::diagnostics::Span;

#[derive(Debug)]
pub enum HirExprLiteral {
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

pub enum HirExprKind {
    Literal(HirExprLiteral),
    Identifier(String),
}

pub struct HirExpr {
    pub kind: HirExprKind,
    span: Span,
}
