use crate::tokens::Token;

pub enum Expr {
    Binary(Box<Binary>)
}

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>
}

impl Binary {
    fn new(left: Expr, op: Token, right: Expr) -> Expr {
        Expr::Binary(Box::new(Binary {
            left: Box::new(left),
            operator: op,
            right: Box::new(right)
        }))
    }
}
