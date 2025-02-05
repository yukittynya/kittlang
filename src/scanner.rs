use crate::tokens::Token;

struct Error {
    pub msg: String,
    pub line: u32,
    pub col: u32
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u32,
    line: u32,
    current: u32,
    err: Option<Error>
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            line: 0,
            current: 0,
            err: None
        }
    }
}
