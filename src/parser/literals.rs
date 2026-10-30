use crate::{
    ast::{ASTExpr, ASTExprKind, ASTExprLiteral},
    lexer::{TType, Token},
    parser::Parser,
};

impl Parser {
    pub fn parse_literal(&mut self) -> Option<ASTExpr> {
        let token = self.current_token()?.clone();
        let span = self.current_token()?.span.clone();

        match token.token_type {
            TType::True => {
                self.advance();
                Some(ASTExpr::new(
                    ASTExprKind::Literal(ASTExprLiteral::Bool(true)),
                    span,
                ))
            }
            TType::False => {
                self.advance();
                Some(ASTExpr::new(
                    ASTExprKind::Literal(ASTExprLiteral::Bool(false)),
                    span,
                ))
            }
            TType::I8Literal
            | TType::U8Literal
            | TType::I16Literal
            | TType::U16Literal
            | TType::I32Literal
            | TType::U32Literal
            | TType::I64Literal
            | TType::U64Literal
            | TType::IntLiteral
            | TType::F32Literal
            | TType::F64Literal
            | TType::FloatLiteral
            | TType::StringLiteral
            | TType::Char8Literal
            | TType::Char16Literal
            | TType::Char32Literal => {
                let lit = self.get_value(&token)?;
                self.advance();
                Some(ASTExpr::new(ASTExprKind::Literal(lit), span))
            }
            _ => {
                self.report(
                    format!("Expected literal, found {:?}", token.token_type),
                    Some(span),
                );
                self.advance();
                None
            }
        }
    }

    pub fn parse_number(&self, num_str: &str) -> Option<i128> {
        if num_str.starts_with("0x") || num_str.starts_with("0X") {
            let hex_str = &num_str[2..];
            i128::from_str_radix(hex_str, 16).ok()
        } else if num_str.starts_with("0b") || num_str.starts_with("0B") {
            let bin_str = &num_str[2..];
            i128::from_str_radix(bin_str, 2).ok()
        } else if num_str.starts_with("0o") || num_str.starts_with("0O") {
            let oct_str = &num_str[2..];
            i128::from_str_radix(oct_str, 8).ok()
        } else {
            num_str.parse::<i128>().ok()
        }
    }

    fn parse_float(&self, num_str: &str) -> Option<f64> {
        num_str.parse::<f64>().ok()
    }

    fn int_literal<T: TryFrom<i128>>(&mut self, token: &Token, ty: &str) -> Option<T> {
        let span = token.span.clone();
        let Some(value) = self.parse_number(&token.lexeme) else {
            self.report("Integer literal is too large".to_string(), Some(span));
            return None;
        };
        match T::try_from(value) {
            Ok(v) => Some(v),
            Err(_) => {
                self.report(
                    format!("Integer literal {} is out of range for {}", value, ty),
                    Some(span),
                );
                None
            }
        }
    }

    fn get_value(&mut self, token: &Token) -> Option<ASTExprLiteral> {
        let lexeme = &token.lexeme;
        match token.token_type {
            TType::I8Literal => Some(ASTExprLiteral::I8(self.int_literal::<i8>(token, "i8")?)),
            TType::U8Literal => Some(ASTExprLiteral::U8(self.int_literal::<u8>(token, "u8")?)),

            TType::I16Literal => Some(ASTExprLiteral::I16(self.int_literal::<i16>(token, "i16")?)),
            TType::U16Literal => Some(ASTExprLiteral::U16(self.int_literal::<u16>(token, "u16")?)),

            TType::I32Literal => Some(ASTExprLiteral::I32(self.int_literal::<i32>(token, "i32")?)),
            TType::U32Literal => Some(ASTExprLiteral::U32(self.int_literal::<u32>(token, "u32")?)),

            TType::I64Literal => Some(ASTExprLiteral::I64(self.int_literal::<i64>(token, "i64")?)),
            TType::U64Literal => Some(ASTExprLiteral::U64(self.int_literal::<u64>(token, "u64")?)),

            TType::IntLiteral => Some(ASTExprLiteral::I32(self.int_literal::<i32>(token, "i32")?)),

            TType::F32Literal => {
                let value = self.parse_float(lexeme)? as f32;
                Some(ASTExprLiteral::F32(value))
            }
            TType::F64Literal => {
                let value = self.parse_float(lexeme)? as f64;
                Some(ASTExprLiteral::F64(value))
            }
            TType::FloatLiteral => {
                let value = self.parse_float(lexeme)? as f32;
                Some(ASTExprLiteral::F32(value))
            }
            TType::StringLiteral => Some(ASTExprLiteral::Str(lexeme.clone())),
            TType::Char8Literal => {
                let ch = lexeme.chars().next()?;
                Some(ASTExprLiteral::Char8(u8::try_from(ch).ok()?))
            }
            TType::Char16Literal => {
                let ch = lexeme.chars().next()?;
                Some(ASTExprLiteral::Char16(u16::try_from(ch).ok()?))
            }
            TType::Char32Literal => Some(ASTExprLiteral::Char32(lexeme.chars().next()?)),
            _ => {
                let span = token.clone().span;
                self.report(
                    format!("Unexpected token type: {:?}", token.token_type),
                    Some(span),
                );
                None
            }
        }
    }
}
