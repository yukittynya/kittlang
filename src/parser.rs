use crate::tokens;

pub struct Parser {
    pub tokens: Vec<tokens::Token>,
    pub current: usize,
}
