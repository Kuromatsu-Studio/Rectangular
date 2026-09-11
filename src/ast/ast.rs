use crate::lexer::TType;

///This is represents binary operators
#[derive(Debug)]
pub enum BinaryOp {
    Add,       //+
    Minus,     //-
    Mul,       //*
    Div,       //`/`
    Quotient,  //'//'
    Gt,        //>
    Gte,       //>=
    Lt,        //<
    Lte,       //<=
    Neq,       //`!=`
    Eq,        //==
    Modulus,   //%
    Assign,    //=
    AddAssign, //+=
    SubAssign, //-=
    MulAssign, //*=
    DivAssign, //`/=`
    Access,    //.
    Invalid,
}

impl BinaryOp {
    pub fn new(ttype: &TType) -> Self {
        match ttype {
            TType::Plus => BinaryOp::Add,
            TType::Minus => BinaryOp::Minus,
            TType::Asterisk => BinaryOp::Mul,
            TType::Slash => BinaryOp::Div,
            TType::DoubleSlash => BinaryOp::Quotient,
            TType::AddAssign => BinaryOp::AddAssign,
            TType::SubAssign => BinaryOp::SubAssign,
            TType::MulAssign => BinaryOp::MulAssign,
            TType::DivAssign => BinaryOp::DivAssign,
            TType::Assign => BinaryOp::Assign,
            TType::Gt => BinaryOp::Gt,
            TType::Lt => BinaryOp::Lt,
            TType::Gte => BinaryOp::Gte,
            TType::Lte => BinaryOp::Lte,
            TType::Neq => BinaryOp::Neq,
            TType::Equality => BinaryOp::Eq,
            TType::Dot => BinaryOp::Access,
            _ => BinaryOp::Invalid,
        }
    }
}

impl BinaryOp {
    pub fn is_valid(ttype: &TType) -> bool {
        match ttype {
            TType::Plus
            | TType::Minus
            | TType::Asterisk
            | TType::AddAssign
            | TType::SubAssign
            | TType::MulAssign
            | TType::DivAssign
            | TType::Slash
            | TType::DoubleSlash
            | TType::Gt
            | TType::Lt
            | TType::Neq
            | TType::Equality
            | TType::Assign
            | TType::Lte
            | TType::Gte
            | TType::Dot => true,
            _ => false,
        }
    }
}

///This represents prefix operators
#[derive(Debug, PartialEq, Eq)]
pub enum UnaryOp {
    Neg, //-
    Not, //`!`
    Invalid,
}

impl UnaryOp {
    pub fn new(ttype: &TType) -> Self {
        match ttype {
            TType::Bang => UnaryOp::Not,
            TType::Minus => UnaryOp::Neg,
            _ => UnaryOp::Invalid,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest = 0,
    Assignment, // =
    Coalesce,   //"??"
    Or,         // ||
    And,        // &&
    Equality,   // == !=
    Comparison, // < > <= >=
    BitwiseOr,  //|
    BitwiseXor, //xor
    BitwiseAnd, //and
    Shift,      //shl,shr
    Term,       // + -
    Factor,     // "* /"
    Call,       // .
    Primary,    //::
}

impl Precedence {
    pub fn token_precedence(ttype: &TType) -> Self {
        match ttype {
            TType::Plus | TType::Minus => Precedence::Term,
            TType::Lt | TType::Gt | TType::Lte | TType::Gte => Precedence::Comparison,
            TType::Neq | TType::Equality => Precedence::Equality,
            TType::Asterisk | TType::Slash => Precedence::Factor,
            TType::Dot => Precedence::Call,
            TType::Assign | TType::AddAssign | TType::SubAssign => Precedence::Assignment,
            _ => Precedence::Lowest,
        }
    }

    pub fn next(self) -> Self {
        match self {
            Precedence::Lowest => Precedence::Assignment,
            Precedence::Assignment => Precedence::Coalesce,
            Precedence::Coalesce => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::BitwiseOr,
            Precedence::BitwiseOr => Precedence::BitwiseXor,
            Precedence::BitwiseXor => Precedence::BitwiseAnd,
            Precedence::BitwiseAnd => Precedence::Shift,
            Precedence::Shift => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => Precedence::Primary,
        }
    }

    fn is_right_associative(op: BinaryOp) -> bool {
        matches!(
            op,
            BinaryOp::Assign | BinaryOp::AddAssign | BinaryOp::SubAssign
        )
    }

    pub fn prec(ttype: &TType) -> Self {
        let op = BinaryOp::new(ttype);
        let op_prec = Precedence::token_precedence(ttype);
        if Precedence::is_right_associative(op) {
            op_prec
        } else {
            op_prec.next()
        }
    }
}
