use crate::tokens::Literal;

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Group(Box<Expr>),
    Literal(Literal),
    Unary(UnaryOp, Box<Expr>)
}

#[derive(Debug)]
pub enum BinaryOpType {
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug)]
pub struct BinaryOp {
    pub op: BinaryOpType,
    pub line: usize,
    pub col: usize
}

#[derive(Debug)]
pub enum UnaryOpType {
    Bang,
    Minus
}

#[derive(Debug)]
pub struct UnaryOp {
    pub op: UnaryOpType,
    pub line: usize,
    pub col: usize
}
