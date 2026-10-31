use std::fmt::{self, write};

use crate::ast::ast::{BinaryOp, UnaryOp};
use crate::ast::{
    ASTConstructorMember, ASTExpr, ASTExprKind, ASTExprLiteral, ASTParam, ASTType, ASTTypeKind,
};
use crate::diagnostics::Span;

fn indent(f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
    for _ in 0..depth {
        write!(f, "    ")?;
    }
    Ok(())
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl fmt::Display for ASTExprLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTExprLiteral::I8(v) => write!(f, "{}i8", v),
            ASTExprLiteral::U8(v) => write!(f, "{}u8", v),
            ASTExprLiteral::I16(v) => write!(f, "{}i16", v),
            ASTExprLiteral::U16(v) => write!(f, "{}u16", v),
            ASTExprLiteral::I32(v) => write!(f, "{}i32", v),
            ASTExprLiteral::U32(v) => write!(f, "{}u32", v),
            ASTExprLiteral::I64(v) => write!(f, "{}i64", v),
            ASTExprLiteral::U64(v) => write!(f, "{}u64", v),
            ASTExprLiteral::F32(v) => write!(f, "{}f32", v),
            ASTExprLiteral::F64(v) => write!(f, "{}f64", v),
            ASTExprLiteral::Bool(v) => write!(f, "{}", v),
            ASTExprLiteral::Str(v) => write!(f, "{:?}", v),
            ASTExprLiteral::Char8(v) => write!(f, "{:?}c8", *v as char),
            ASTExprLiteral::Char16(v) => {
                write!(
                    f,
                    "{:?}c16",
                    char::from_u32(*v as u32).unwrap_or('\u{FFFD}')
                )
            }
            ASTExprLiteral::Char32(v) => write!(f, "{:?}c32", v),
        }
    }
}

impl BinaryOp {
    fn as_str(&self) -> &str {
        match self {
            BinaryOp::Add => "+",
            BinaryOp::Minus => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Gt => ">",
            BinaryOp::Gte => ">=",
            BinaryOp::Lt => "<",
            BinaryOp::Lte => "<=",
            BinaryOp::Neq => "!=",
            BinaryOp::Eq => "==",
            BinaryOp::Modulus => "%",
            BinaryOp::Assign => "=",
            BinaryOp::AddAssign => "+=",
            BinaryOp::SubAssign => "-=",
            BinaryOp::MulAssign => "*=",
            BinaryOp::DivAssign => "/=",
            BinaryOp::Access => ".",
            BinaryOp::LogicalOr => "||",
            BinaryOp::LogicalAnd => "&&",
            BinaryOp::BitAnd => "&",
            BinaryOp::BitOr => "|",
            BinaryOp::Shr => ">>",
            BinaryOp::Shl => "<<",
            BinaryOp::Invalid => "<??>",
        }
    }
}

impl UnaryOp {
    fn as_str(&self) -> &str {
        match self {
            UnaryOp::Neg => "-",
            UnaryOp::Not => "!",
            UnaryOp::Invalid => "<??>",
        }
    }
}

impl fmt::Display for ASTType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ASTTypeKind::I8 => write!(f, "i8"),
            ASTTypeKind::U8 => write!(f, "u8"),
            ASTTypeKind::I16 => write!(f, "i16"),
            ASTTypeKind::U16 => write!(f, "u16"),
            ASTTypeKind::I32 => write!(f, "i32"),
            ASTTypeKind::U32 => write!(f, "u32"),
            ASTTypeKind::I64 => write!(f, "i64"),
            ASTTypeKind::U64 => write!(f, "u64"),
            ASTTypeKind::F32 => write!(f, "f32"),
            ASTTypeKind::F64 => write!(f, "f64"),
            ASTTypeKind::Bool => write!(f, "bool"),
            ASTTypeKind::Unit => write!(f, "unit"),
            ASTTypeKind::Type => write!(f, "type"),
            ASTTypeKind::Expr(inner) => write!(f, "{}", inner),
            ASTTypeKind::Pointer(inner) => write!(f, "*{}", inner),
            ASTTypeKind::None => write!(f, "<?>"),
            ASTTypeKind::Custom(name) => write!(f, "{}", name),
        }
    }
}

impl fmt::Display for ASTParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Param {}: {}", self.name.as_ref(), self.ty.as_ref())
    }
}

impl fmt::Display for ASTConstructorMember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Member {} : {}", self.name.as_ref(), self.val.as_ref())
    }
}

impl ASTExpr {
    fn fmt_pretty(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
        match &self.kind {
            ASTExprKind::Literal(lit) => {
                indent(f, depth)?;
                write!(f, "Literal({}) @ {}", lit, self.span)
            }

            ASTExprKind::TypeExpr(ty) => write!(f, "Type Expr: {}", ty),

            ASTExprKind::Lambda {
                params,
                ret_ty,
                body,
            } => {
                indent(f, depth + 1)?;
                write!(f, "Lambda: ")?;

                write!(f, "(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ")")?;

                if let Some(ty) = ret_ty {
                    write!(f, " Return_Type:  {}", ty)?;
                }

                write!(f, " -> Body: {{")?;
                write!(f, "{}", body)?;
                write!(f, "}}")
            }

            ASTExprKind::DollarExpr(expr) => {
                write!(f, "DollarExpr: ")?;
                indent(f, depth + 1)?;
                expr.fmt_pretty(f, depth)
            }

            ASTExprKind::DoubleDollar(expr) => {
                write!(f, "DoubleDollarExpr: ")?;
                indent(f, depth + 1)?;
                expr.fmt_pretty(f, depth)
            }

            ASTExprKind::Bind {
                mutable,
                names,
                ty,
                init,
            } => {
                indent(f, depth + 1)?;
                write!(f, "Bind: ")?;

                write!(f, "Mutable: {} ", mutable)?;

                for (i, name) in names.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "Name {} ", name)?;
                }

                if let Some(ty_val) = ty {
                    write!(f, "Type: {}", ty_val)?;
                }

                write!(f, " Value: {{")?;

                if let Some(init_val) = init {
                    write!(f, "{}", init_val)?;
                }

                write!(f, "}}")
            }

            ASTExprKind::Struct { members } => {
                write!(f, "Struct: {{")?;
                for (i, member) in members.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "Member {} ", member)?;
                }
                write!(f, "}}")
            }

            ASTExprKind::StructConstructor { name, members } => {
                write!(f, " Struct Constructor: {} (", name)?;
                indent(f, depth)?;
                for (i, member) in members.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, " {} ", member)?;
                }

                write!(f, ")")
            }

            ASTExprKind::Break => write!(f, "Break @{}", self.span),
            ASTExprKind::Continue => write!(f, "Continue @{}", self.span),

            ASTExprKind::IfExpr {
                condition,
                if_body,
                else_body,
            } => {
                write!(f, "IfExpr: {{")?;
                indent(f, depth)?;
                write!(f, "Condition: {} ", condition)?;
                write!(f, "Body: {} ", if_body)?;
                if let Some(else_bod) = else_body {
                    write!(f, "Else: {} ", else_bod)?;
                }
                write!(f, "}}")
            }

            ASTExprKind::WhileExpr { condition, body } => {
                write!(f, "WhileExpr: ")?;
                indent(f, depth)?;
                write!(f, "Condition {} ", condition)?;
                write!(f, "Body: {}", body)
            }

            ASTExprKind::Identifier(name) => {
                indent(f, depth)?;
                write!(f, "Identifier({}) @ {}", name, self.span)
            }

            ASTExprKind::Unary(op, expr) => {
                indent(f, depth)?;
                write!(f, "Unary({}) @ {} {{", op.as_str(), self.span)?;
                write!(
                    f,
                    "
"
                )?;
                expr.fmt_pretty(f, depth + 1)?;
                write!(
                    f,
                    "
"
                )?;
                indent(f, depth)?;
                write!(f, "}}")
            }

            ASTExprKind::Binary(left, op, right) => {
                indent(f, depth)?;
                write!(f, "Binary({}) @ {} {{", op.as_str(), self.span)?;
                write!(
                    f,
                    "
"
                )?;
                left.fmt_pretty(f, depth + 1)?;
                write!(
                    f,
                    "
"
                )?;
                right.fmt_pretty(f, depth + 1)?;
                write!(
                    f,
                    "
"
                )?;
                indent(f, depth)?;
                write!(f, "}}")
            }

            ASTExprKind::Block(exprs) => {
                indent(f, depth)?;
                if exprs.is_empty() {
                    write!(f, "Block {{}} @ {}", self.span)
                } else {
                    write!(f, "Block @ {} {{", self.span)?;
                    for e in exprs {
                        write!(
                            f,
                            "
"
                        )?;
                        e.fmt_pretty(f, depth + 1)?;
                    }
                    write!(
                        f,
                        "
"
                    )?;
                    indent(f, depth)?;
                    write!(f, "}}")
                }
            }

            ASTExprKind::Call(func, args) => {
                indent(f, depth)?;
                write!(f, "Call @ {} {{", self.span)?;
                write!(
                    f,
                    "
"
                )?;
                indent(f, depth + 1)?;
                write!(
                    f,
                    "Func:
"
                )?;
                func.fmt_pretty(f, depth + 2)?;
                if !args.is_empty() {
                    write!(
                        f,
                        "
"
                    )?;
                    indent(f, depth + 1)?;
                    write!(f, "Args:")?;
                    for arg in args {
                        write!(
                            f,
                            "
"
                        )?;
                        arg.fmt_pretty(f, depth + 2)?;
                    }
                }
                write!(
                    f,
                    "
"
                )?;
                indent(f, depth)?;
                write!(f, "}}")
            }

            ASTExprKind::Index(base, idx) => {
                indent(f, depth)?;
                write!(f, "Index @ {} {{", self.span)?;
                write!(
                    f,
                    "
"
                )?;
                indent(f, depth + 1)?;
                write!(
                    f,
                    "Base:
"
                )?;
                base.fmt_pretty(f, depth + 2)?;
                write!(
                    f,
                    "
"
                )?;
                indent(f, depth + 1)?;
                write!(
                    f,
                    "Index:
"
                )?;
                idx.fmt_pretty(f, depth + 2)?;
                write!(
                    f,
                    "
"
                )?;
                indent(f, depth)?;
                write!(f, "}}")
            }

            ASTExprKind::Array(exprs) => {
                indent(f, depth)?;
                if exprs.is_empty() {
                    write!(f, "Array[] @ {}", self.span)
                } else {
                    write!(f, "Array @ {} {{", self.span)?;
                    for e in exprs {
                        write!(
                            f,
                            "
"
                        )?;
                        e.fmt_pretty(f, depth + 1)?;
                    }
                    write!(
                        f,
                        "
"
                    )?;
                    indent(f, depth)?;
                    write!(f, "}}")
                }
            }

            ASTExprKind::Return(expr) => {
                indent(f, depth)?;
                write!(f, "Return @ {} ", self.span)?;
                write!(
                    f,
                    "
"
                )?;

                if let Some(expr_val) = expr {
                    expr_val.fmt_pretty(f, depth + 1)?;
                    write!(
                        f,
                        "
"
                    )?;

                    indent(f, depth)?;
                }

                write!(f, "")
            }
        }
    }
}

impl fmt::Display for ASTExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_pretty(f, 0)
    }
}

pub fn pretty_module(exprs: Vec<ASTExpr>) -> String {
    let mut out = String::new();
    for (i, stmt) in exprs.iter().enumerate() {
        if i > 0 {
            out.push_str(
                "

",
            );
        }
        out.push_str(&format!("{}", stmt));
    }
    out
}
