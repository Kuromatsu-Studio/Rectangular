use std::fmt;

// Adjust these imports to match your crate structure
use crate::ast::ast::{BinaryOp, UnaryOp};
use crate::ast::{
    ASTParam, ASTType, ASTTypeKind, DeclPattern, DeclPatternKind, Expr, ExprKind, ExprLiteral,
    Stmt, StmtKind,
};
use crate::diagnostics::Span;

fn indent(f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
    for _ in 0..depth {
        write!(f, "    ")?;
    }
    Ok(())
}

fn indent_str(depth: usize) -> String {
    "    ".repeat(depth)
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl fmt::Display for ExprLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExprLiteral::I8(v) => write!(f, "{}i8", v),
            ExprLiteral::U8(v) => write!(f, "{}u8", v),
            ExprLiteral::I16(v) => write!(f, "{}i16", v),
            ExprLiteral::U16(v) => write!(f, "{}u16", v),
            ExprLiteral::I32(v) => write!(f, "{}i32", v),
            ExprLiteral::U32(v) => write!(f, "{}u32", v),
            ExprLiteral::I64(v) => write!(f, "{}i64", v),
            ExprLiteral::U64(v) => write!(f, "{}u64", v),
            ExprLiteral::F32(v) => write!(f, "{}f32", v),
            ExprLiteral::F64(v) => write!(f, "{}f64", v),
            ExprLiteral::Bool(v) => write!(f, "{}", v),
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
            BinaryOp::Quotient => "//",
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
            ASTTypeKind::Str => write!(f, "str"),
            ASTTypeKind::Unit => write!(f, "()"),
            ASTTypeKind::None => write!(f, "<?>"),
            ASTTypeKind::Custom(name) => write!(f, "{}", name),
            ASTTypeKind::Func { params, ret_ty } => {
                write!(f, "func(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ")")?;
                if let Some(ret) = ret_ty.as_ref() {
                    write!(f, ": {}", ret)?;
                }
                Ok(())
            }
            ASTTypeKind::Tuple(types) => {
                write!(f, "(")?;
                for (i, t) in types.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", t)?;
                }
                write!(f, ")")
            }
            ASTTypeKind::Array(ty, size) => {
                write!(f, "[{}; {}]", ty, size)
            }
        }
    }
}

impl fmt::Display for ASTParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.ty)
    }
}

impl fmt::Display for DeclPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            DeclPatternKind::Name(expr) => {
                write!(f, "{}", expr)
            }
            DeclPatternKind::Wildcard => {
                write!(f, "_")
            }
            DeclPatternKind::Tuple(patterns) => {
                write!(f, "(")?;
                for (i, p) in patterns.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ")")
            }
            DeclPatternKind::Record { name, fields } => {
                write!(f, "{} {{", name)?;
                for (i, (field, binding)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    if let Some(bind) = binding {
                        write!(f, "{}: {}", field, bind)?;
                    } else {
                        write!(f, "{}", field)?;
                    }
                }
                write!(f, "}}")
            }
        }
    }
}

impl Expr {
    /// Pretty-print with indentation. Call from Display with depth=0.
    fn fmt_pretty(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
        match &self.kind {
            ExprKind::Literal(lit) => {
                indent(f, depth)?;
                write!(f, "Literal({}) @ {}", lit, self.span)
            }

            ExprKind::Identifier(name) => {
                indent(f, depth)?;
                write!(f, "Identifier({}) @ {}", name, self.span)
            }

            ExprKind::Unary(op, expr) => {
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

            ExprKind::Binary(left, op, right) => {
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

            ExprKind::Block(exprs) => {
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

            ExprKind::Let { pattern, ty, init } => {
                indent(f, depth)?;
                write!(f, "Let @ {} {{", self.span)?;
                write!(
                    f,
                    "
"
                )?;
                indent(f, depth + 1)?;
                write!(f, "Pattern: {}", pattern)?;
                if let Some(t) = ty {
                    write!(
                        f,
                        "
"
                    )?;
                    indent(f, depth + 1)?;
                    write!(f, "Type: {}", t)?;
                }
                write!(
                    f,
                    "
"
                )?;
                indent(f, depth + 1)?;
                write!(
                    f,
                    "Init:
"
                )?;
                init.fmt_pretty(f, depth + 2)?;
                write!(
                    f,
                    "
"
                )?;
                indent(f, depth)?;
                write!(f, "}}")
            }

            ExprKind::If {
                cond,
                then_branch,
                else_branch,
            } => {
                indent(f, depth)?;
                write!(f, "If @ {} {{", self.span)?;
                write!(
                    f,
                    "
"
                )?;
                indent(f, depth + 1)?;
                write!(
                    f,
                    "Cond:
"
                )?;
                cond.fmt_pretty(f, depth + 2)?;
                write!(
                    f,
                    "
"
                )?;
                indent(f, depth + 1)?;
                write!(
                    f,
                    "Then:
"
                )?;
                then_branch.fmt_pretty(f, depth + 2)?;
                if let Some(else_expr) = else_branch.as_ref() {
                    write!(
                        f,
                        "
"
                    )?;
                    indent(f, depth + 1)?;
                    write!(
                        f,
                        "Else:
"
                    )?;
                    else_expr.fmt_pretty(f, depth + 2)?;
                }
                write!(
                    f,
                    "
"
                )?;
                indent(f, depth)?;
                write!(f, "}}")
            }

            ExprKind::While { cond, body } => {
                indent(f, depth)?;
                write!(f, "While @ {} {{", self.span)?;
                write!(
                    f,
                    "
"
                )?;
                indent(f, depth + 1)?;
                write!(
                    f,
                    "Cond:
"
                )?;
                cond.fmt_pretty(f, depth + 2)?;
                write!(
                    f,
                    "
"
                )?;
                indent(f, depth + 1)?;
                write!(
                    f,
                    "Body:
"
                )?;
                body.fmt_pretty(f, depth + 2)?;
                write!(
                    f,
                    "
"
                )?;
                indent(f, depth)?;
                write!(f, "}}")
            }

            ExprKind::Lambda {
                params,
                ret_ty,
                block,
            } => {
                indent(f, depth)?;
                write!(f, "Lambda @ {} {{", self.span)?;
                write!(
                    f,
                    "
"
                )?;
                indent(f, depth + 1)?;
                write!(f, "Params: [")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, "]")?;
                if let Some(ret) = ret_ty {
                    write!(
                        f,
                        "
"
                    )?;
                    indent(f, depth + 1)?;
                    write!(f, "Ret: {}", ret)?;
                }
                write!(
                    f,
                    "
"
                )?;
                indent(f, depth + 1)?;
                write!(
                    f,
                    "Body:
"
                )?;
                block.fmt_pretty(f, depth + 2)?;
                write!(
                    f,
                    "
"
                )?;
                indent(f, depth)?;
                write!(f, "}}")
            }

            ExprKind::Call(func, args) => {
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

            ExprKind::Index(base, idx) => {
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

            ExprKind::Tuple(exprs) => {
                indent(f, depth)?;
                if exprs.is_empty() {
                    write!(f, "Tuple() @ {}", self.span)
                } else {
                    write!(f, "Tuple @ {} {{", self.span)?;
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

            ExprKind::Record { name, fields } => {
                indent(f, depth)?;
                write!(f, "Record({}) @ {} {{", name, self.span)?;
                for (field, value) in fields {
                    write!(
                        f,
                        "
"
                    )?;
                    indent(f, depth + 1)?;
                    write!(f, "{}: ", field)?;
                    // Inline the value if simple, else newline + indent
                    value.fmt_pretty(f, 0)?;
                }
                write!(
                    f,
                    "
"
                )?;
                indent(f, depth)?;
                write!(f, "}}")
            }

            ExprKind::Array(exprs) => {
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

            ExprKind::Return(expr) => {
                indent(f, depth)?;
                write!(f, "Return @ {} {{", self.span)?;
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
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_pretty(f, 0)
    }
}

impl Stmt {
    fn fmt_pretty(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
        match &self.kind {
            StmtKind::Record { name, params } => {
                indent(f, depth)?;
                write!(f, "Record({}) @ {} {{", name, self.span)?;
                if !params.is_empty() {
                    for p in params {
                        write!(
                            f,
                            "
"
                        )?;
                        indent(f, depth + 1)?;
                        write!(f, "{}", p)?;
                    }
                    write!(
                        f,
                        "
"
                    )?;
                    indent(f, depth)?;
                }
                write!(f, "}}")
            }

            StmtKind::Enum { name, block } => {
                indent(f, depth)?;
                write!(f, "Enum({}) @ {} {{", name, self.span)?;
                for variant in block {
                    write!(
                        f,
                        "
"
                    )?;
                    indent(f, depth + 1)?;
                    write!(f, "{}", variant)?;
                }
                write!(
                    f,
                    "
"
                )?;
                indent(f, depth)?;
                write!(f, "}}")
            }

            StmtKind::Let(expr) => {
                // The inner expr should be an ExprKind::Let
                indent(f, depth)?;
                write!(
                    f,
                    "Stmt::Let @ {}
",
                    self.span
                )?;
                expr.fmt_pretty(f, depth)
            }

            StmtKind::FuncDef {
                name,
                params,
                ret_ty,
                body,
            } => {
                indent(f, depth)?;
                write!(f, "FuncDef({}) @ {} {{", name, self.span)?;
                write!(
                    f,
                    "
"
                )?;
                indent(f, depth + 1)?;
                write!(f, "Params: [")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, "]")?;
                if let Some(ret) = ret_ty.as_ref() {
                    write!(
                        f,
                        "
"
                    )?;
                    indent(f, depth + 1)?;
                    write!(f, "Ret: {}", ret)?;
                }
                write!(
                    f,
                    "
"
                )?;
                indent(f, depth + 1)?;
                write!(
                    f,
                    "Body:
"
                )?;
                body.fmt_pretty(f, depth + 2)?;
                write!(
                    f,
                    "
"
                )?;
                indent(f, depth)?;
                write!(f, "}}")
            }
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_pretty(f, 0)
    }
}

pub fn pretty_module(stmts: Vec<Stmt>) -> String {
    let mut out = String::new();
    for (i, stmt) in stmts.iter().enumerate() {
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
