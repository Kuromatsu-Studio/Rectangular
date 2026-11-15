use std::fmt::Write;
use std::fmt;

use crate::hir::{
    expr::{ComptimeKind, HIRExprKind, HIRExprLiteral, HIRParam},
    hir::{AttrId, ExprId, HIRModule, TypeNodeId},
    types::{HIRTypeKind, Prim},
};

fn ind(depth: usize) -> String {
    "    ".repeat(depth)
}

impl fmt::Display for Prim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prim::I8 => write!(f, "i8"),
            Prim::U8 => write!(f, "u8"),
            Prim::I16 => write!(f, "i16"),
            Prim::U16 => write!(f, "u16"),
            Prim::I32 => write!(f, "i32"),
            Prim::U32 => write!(f, "u32"),
            Prim::I64 => write!(f, "i64"),
            Prim::U64 => write!(f, "u64"),
            Prim::F32 => write!(f, "f32"),
            Prim::F64 => write!(f, "f64"),
            Prim::C8 => write!(f, "c8"),
            Prim::C16 => write!(f, "c16"),
            Prim::C32 => write!(f, "c32"),
            Prim::Bool => write!(f, "bool"),
            Prim::Unit => write!(f, "unit"),
            Prim::Type => write!(f, "type"),
            Prim::Error => write!(f, "<?>"),
        }
    }
}

impl fmt::Display for HIRExprLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HIRExprLiteral::I8(v) => write!(f, "{}i8", v),
            HIRExprLiteral::U8(v) => write!(f, "{}u8", v),
            HIRExprLiteral::I16(v) => write!(f, "{}i16", v),
            HIRExprLiteral::U16(v) => write!(f, "{}u16", v),
            HIRExprLiteral::I32(v) => write!(f, "{}i32", v),
            HIRExprLiteral::U32(v) => write!(f, "{}u32", v),
            HIRExprLiteral::I64(v) => write!(f, "{}i64", v),
            HIRExprLiteral::U64(v) => write!(f, "{}u64", v),
            HIRExprLiteral::F32(v) => write!(f, "{}f32", v),
            HIRExprLiteral::F64(v) => write!(f, "{}f64", v),
            HIRExprLiteral::Bool(v) => write!(f, "{}", v),
            HIRExprLiteral::Str(v) => write!(f, "{:?}", v),
            HIRExprLiteral::Char8(v) => write!(f, "{:?}c8", *v as char),
            HIRExprLiteral::Char16(v) => {
                write!(
                    f,
                    "{:?}c16",
                    char::from_u32(*v as u32).unwrap_or('\u{FFFD}')
                )
            }
            HIRExprLiteral::Char32(v) => write!(f, "{:?}c32", v),
        }
    }
}

impl fmt::Display for ComptimeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComptimeKind::Read => write!(f, "$"),
            ComptimeKind::Evaluate => write!(f, "$$"),
        }
    }
}

fn fmt_type(w: &mut dyn fmt::Write, module: &HIRModule, id: TypeNodeId) -> fmt::Result {
    let ty = &module.types[id.0];
    match &ty.kind {
        HIRTypeKind::Primitive(p) => write!(w, "{}", p),
        HIRTypeKind::Path(path) => write!(w, "{}", path.join(".")),
        HIRTypeKind::Pointer(inner) => {
            write!(w, "*")?;
            fmt_type(w, module, *inner)
        }
        HIRTypeKind::Function {
            params,
            return_ty,
        } => {
            write!(w, "fn(")?;
            for (i, p) in params.iter().enumerate() {
                if i > 0 {
                    write!(w, ", ")?;
                }
                fmt_type(w, module, *p)?;
            }
            write!(w, "): ")?;
            fmt_type(w, module, *return_ty)
        }
        HIRTypeKind::Applied { callee, args } => {
            fmt_type(w, module, *callee)?;
            write!(w, "(")?;
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    write!(w, ", ")?;
                }
                fmt_type(w, module, *arg)?;
            }
            write!(w, ")")
        }
        HIRTypeKind::Array { ty, length } => {
            write!(w, "[")?;
            fmt_type(w, module, *ty)?;
            write!(w, "; ")?;
            fmt_expr(w, module, *length, 0)?;
            write!(w, "]")
        }
    }
}

fn fmt_attr(w: &mut dyn fmt::Write, module: &HIRModule, id: AttrId) -> fmt::Result {
    let attr = &module.attrs[id.0];
    write!(w, "@{}", attr.path.join("."))?;
    if !attr.args.is_empty() {
        write!(w, "(")?;
        for (i, arg) in attr.args.iter().enumerate() {
            if i > 0 {
                write!(w, ", ")?;
            }
            fmt_expr(w, module, *arg, 0)?;
        }
        write!(w, ")")?;
    }
    Ok(())
}

fn fmt_param(
    w: &mut dyn fmt::Write,
    module: &HIRModule,
    p: &HIRParam,
) -> fmt::Result {
    if !p.attributes.is_empty() {
        write!(w, "[")?;
        for (i, a) in p.attributes.iter().enumerate() {
            if i > 0 {
                write!(w, ", ")?;
            }
            fmt_attr(w, module, *a)?;
        }
        write!(w, "] ")?;
    }
    write!(w, "{}: ", p.name)?;
    fmt_type(w, module, p.ty)
}

fn fmt_expr(
    w: &mut dyn fmt::Write,
    module: &HIRModule,
    id: ExprId,
    depth: usize,
) -> fmt::Result {
    let expr = &module.exprs[id.0];
    let pad = ind(depth);
    match &expr.kind {
        HIRExprKind::Literal(lit) => {
            write!(w, "{}Literal({}) @ {}", pad, lit, expr.span)
        }
        HIRExprKind::Identifier(name) => {
            write!(w, "{}Identifier({}) @ {}", pad, name, expr.span)
        }
        HIRExprKind::Break => write!(w, "{}Break @ {}", pad, expr.span),
        HIRExprKind::Continue => write!(w, "{}Continue @ {}", pad, expr.span),
        HIRExprKind::Return(inner) => match inner {
            Some(i) => {
                writeln!(w, "{}Return @ {} {{", pad, expr.span)?;
                fmt_expr(w, module, *i, depth + 1)?;
                writeln!(w)?;
                write!(w, "{}}}", pad)
            }
            None => write!(w, "{}Return @ {} (void)", pad, expr.span),
        },
        HIRExprKind::Unary { op, operand } => {
            writeln!(w, "{}Unary({:?}) @ {} {{", pad, op, expr.span)?;
            fmt_expr(w, module, *operand, depth + 1)?;
            writeln!(w)?;
            write!(w, "{}}}", pad)
        }
        HIRExprKind::Binary { op, lhs, rhs } => {
            writeln!(w, "{}Binary({:?}) @ {} {{", pad, op, expr.span)?;
            fmt_expr(w, module, *lhs, depth + 1)?;
            writeln!(w)?;
            fmt_expr(w, module, *rhs, depth + 1)?;
            writeln!(w)?;
            write!(w, "{}}}", pad)
        }
        HIRExprKind::Call { callee, args } => {
            writeln!(w, "{}Call @ {} {{", pad, expr.span)?;
            write!(w, "{}    Func: ", ind(depth + 1))?;
            fmt_expr(w, module, *callee, depth + 2)?;
            if !args.is_empty() {
                writeln!(w)?;
                write!(w, "{}    Args: ", ind(depth + 1))?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        writeln!(w)?;
                        write!(w, "{}    ", ind(depth + 1))?;
                    }
                    fmt_expr(w, module, *arg, depth + 2)?;
                }
            }
            writeln!(w)?;
            write!(w, "{}}}", pad)
        }
        HIRExprKind::Index { base, index } => {
            writeln!(w, "{}Index @ {} {{", pad, expr.span)?;
            write!(w, "{}    Base: ", ind(depth + 1))?;
            fmt_expr(w, module, *base, depth + 2)?;
            writeln!(w)?;
            write!(w, "{}    Index: ", ind(depth + 1))?;
            fmt_expr(w, module, *index, depth + 2)?;
            writeln!(w)?;
            write!(w, "{}}}", pad)
        }
        HIRExprKind::Access { left, right } => {
            writeln!(w, "{}Access @ {} {{", pad, expr.span)?;
            write!(w, "{}    Left: ", ind(depth + 1))?;
            fmt_expr(w, module, *left, depth + 2)?;
            writeln!(w)?;
            write!(w, "{}    Right: ", ind(depth + 1))?;
            fmt_expr(w, module, *right, depth + 2)?;
            writeln!(w)?;
            write!(w, "{}}}", pad)
        }
        HIRExprKind::Dereference(inner) => {
            writeln!(w, "{}Dereference @ {} {{", pad, expr.span)?;
            fmt_expr(w, module, *inner, depth + 1)?;
            writeln!(w)?;
            write!(w, "{}}}", pad)
        }
        HIRExprKind::AddressOf(inner) => {
            writeln!(w, "{}AddressOf @ {} {{", pad, expr.span)?;
            fmt_expr(w, module, *inner, depth + 1)?;
            writeln!(w)?;
            write!(w, "{}}}", pad)
        }
        HIRExprKind::Defer(inner) => {
            writeln!(w, "{}Defer @ {} {{", pad, expr.span)?;
            fmt_expr(w, module, *inner, depth + 1)?;
            writeln!(w)?;
            write!(w, "{}}}", pad)
        }
        HIRExprKind::Comptime { kind, expr: inner } => {
            writeln!(w, "{}Comptime({}) @ {} {{", pad, kind, expr.span)?;
            fmt_expr(w, module, *inner, depth + 1)?;
            writeln!(w)?;
            write!(w, "{}}}", pad)
        }
        HIRExprKind::Array(elements) => {
            if elements.is_empty() {
                write!(w, "{}Array[] @ {}", pad, expr.span)
            } else {
                writeln!(w, "{}Array @ {} {{", pad, expr.span)?;
                for (i, e) in elements.iter().enumerate() {
                    if i > 0 {
                        writeln!(w)?;
                    }
                    fmt_expr(w, module, *e, depth + 1)?;
                }
                writeln!(w)?;
                write!(w, "{}}}", pad)
            }
        }
        HIRExprKind::Block(exprs) => {
            if exprs.is_empty() {
                write!(w, "{}Block {{}} @ {}", pad, expr.span)
            } else {
                writeln!(w, "{}Block @ {} {{", pad, expr.span)?;
                for (i, e) in exprs.iter().enumerate() {
                    if i > 0 {
                        writeln!(w)?;
                    }
                    fmt_expr(w, module, *e, depth + 1)?;
                }
                writeln!(w)?;
                write!(w, "{}}}", pad)
            }
        }
        HIRExprKind::Bind {
            attrs,
            name,
            ty,
            init,
        } => {
            writeln!(w, "{}Bind @ {} {{", pad, expr.span)?;
            write!(w, "{}    name: {}", ind(depth + 1), name)?;
            if let Some(t) = ty {
                writeln!(w)?;
                write!(w, "{}    type: ", ind(depth + 1))?;
                fmt_type(w, module, *t)?;
            }
            if !attrs.is_empty() {
                writeln!(w)?;
                write!(w, "{}    attrs: ", ind(depth + 1))?;
                for (i, a) in attrs.iter().enumerate() {
                    if i > 0 {
                        write!(w, ", ")?;
                    }
                    fmt_attr(w, module, *a)?;
                }
            }
            match init {
                Some(i) => {
                    writeln!(w)?;
                    write!(w, "{}    init: ", ind(depth + 1))?;
                    fmt_expr(w, module, *i, depth + 2)?;
                }
                None => {}
            }
            writeln!(w)?;
            write!(w, "{}}}", pad)
        }
        HIRExprKind::Lambda {
            params,
            ret,
            body,
        } => {
            write!(w, "{}Lambda @ {} (", pad, expr.span)?;
            for (i, p) in params.iter().enumerate() {
                if i > 0 {
                    write!(w, ", ")?;
                }
                fmt_param(w, module, p)?;
            }
            write!(w, ")")?;
            if let Some(t) = ret {
                write!(w, ": ")?;
                fmt_type(w, module, *t)?;
            }
            writeln!(w, " -> {{")?;
            fmt_expr(w, module, *body, depth + 1)?;
            writeln!(w)?;
            write!(w, "{}}}", pad)
        }
        HIRExprKind::LambdaDecl { params, ret } => {
            write!(w, "{}LambdaDecl @ {} (", pad, expr.span)?;
            for (i, p) in params.iter().enumerate() {
                if i > 0 {
                    write!(w, ", ")?;
                }
                fmt_param(w, module, p)?;
            }
            write!(w, "): ")?;
            fmt_type(w, module, *ret)
        }
        HIRExprKind::If {
            condition,
            if_body,
            else_body,
        } => {
            writeln!(w, "{}If @ {} {{", pad, expr.span)?;
            write!(w, "{}    Condition: ", ind(depth + 1))?;
            fmt_expr(w, module, *condition, depth + 2)?;
            writeln!(w)?;
            write!(w, "{}    Body: ", ind(depth + 1))?;
            fmt_expr(w, module, *if_body, depth + 2)?;
            if let Some(el) = else_body {
                writeln!(w)?;
                write!(w, "{}    Else: ", ind(depth + 1))?;
                fmt_expr(w, module, *el, depth + 2)?;
            }
            writeln!(w)?;
            write!(w, "{}}}", pad)
        }
        HIRExprKind::While { condition, body } => {
            writeln!(w, "{}While @ {} {{", pad, expr.span)?;
            write!(w, "{}    Condition: ", ind(depth + 1))?;
            fmt_expr(w, module, *condition, depth + 2)?;
            writeln!(w)?;
            write!(w, "{}    Body: ", ind(depth + 1))?;
            fmt_expr(w, module, *body, depth + 2)?;
            writeln!(w)?;
            write!(w, "{}}}", pad)
        }
        HIRExprKind::Struct { members } => {
            if members.is_empty() {
                write!(w, "{}Struct {{}} @ {}", pad, expr.span)
            } else {
                writeln!(w, "{}Struct @ {} {{", pad, expr.span)?;
                for (i, m) in members.iter().enumerate() {
                    if i > 0 {
                        writeln!(w)?;
                    }
                    write!(w, "{}    ", ind(depth + 1))?;
                    fmt_param(w, module, m)?;
                }
                writeln!(w)?;
                write!(w, "{}}}", pad)
            }
        }
        HIRExprKind::Constructor { name, members } => {
            writeln!(w, "{}Constructor {} @ {} {{", pad, name, expr.span)?;
            for (i, m) in members.iter().enumerate() {
                if i > 0 {
                    writeln!(w)?;
                }
                write!(w, "{}    Member {}: ", ind(depth + 1), m.name)?;
                fmt_expr(w, module, m.val, depth + 2)?;
            }
            writeln!(w)?;
            write!(w, "{}}}", pad)
        }
        HIRExprKind::TypeValue(ty) => {
            write!(w, "{}TypeValue(", pad)?;
            fmt_type(w, module, *ty)?;
            write!(w, ") @ {}", expr.span)
        }
    }
}

pub fn pretty_module(module: &HIRModule) -> String {
    let mut out = String::new();

    writeln!(out, "Roots:").unwrap();
    for r in &module.roots {
        let expr = &module.exprs[r.0];
        writeln!(out, "  expr[{}] @ {}", r.0, expr.span).unwrap();
    }

    writeln!(out).unwrap();
    writeln!(out, "Expressions:").unwrap();
    for expr in &module.exprs {
        fmt_expr(&mut out, module, expr.id, 0).unwrap();
        writeln!(out).unwrap();
    }

    if !module.types.is_empty() {
        writeln!(out).unwrap();
        writeln!(out, "Types:").unwrap();
        for ty in &module.types {
            write!(out, "type[{}]: ", ty.id.0).unwrap();
            fmt_type(&mut out, module, ty.id).unwrap();
            writeln!(out, " @ {}", ty.span).unwrap();
        }
    }

    if !module.attrs.is_empty() {
        writeln!(out).unwrap();
        writeln!(out, "Attributes:").unwrap();
        for attr in &module.attrs {
            write!(out, "attr[{}]: ", attr.id.0).unwrap();
            fmt_attr(&mut out, module, attr.id).unwrap();
            writeln!(out, " @ {}", attr.span).unwrap();
        }
    }

    out
}
