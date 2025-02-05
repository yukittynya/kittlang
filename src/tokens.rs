use std::fmt;

#[allow(dead_code)]
#[derive(strum_macros::Display)]
#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    //single chars
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Star, Slash, SemiColon, 

    //comparision chars
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    //literals
    Identifier, String, Number,

    //keywords
    Let, Mut, Struct, Class, If, Else, Null, 
    True, False, And, Or, Return, This,
    Fn, While, For, Print, Super,

    EOF
}

#[derive(Debug)]
pub enum Literal {
    Indentifier(String), 
    String(String),
    Number(f64)
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: Vec<u8>,
    pub literal: Option<Literal>,
    pub line: usize 
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: Vec<u8>, literal: Option<Literal>, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line
        }
    }
}
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Token [Type: {:?}, Lexeme: \"{}\", Literal: {:?}, Line: {:?}]",
            self.token_type,
            String::from_utf8(self.lexeme.clone()).unwrap(),
            self.literal,
            self.line
        )
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Token [Type: {:?}, Lexeme: \"{}\", Literal: {:?}, Line: {:?}]",
            self.token_type,
            String::from_utf8(self.lexeme.clone()).unwrap(),
            self.literal,
            self.line
        )
    }
}
