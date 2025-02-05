#[derive(strum_macros::Display)]
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

pub enum Literal {
    Indentifier(String), 
    String(String),
    Number(i32)
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: i32
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Literal>, line: i32) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line
        }
    }

    pub fn to_string(self) -> String {
        let res = format!("{} is {} on line {}", self.lexeme, self.token_type, self.line);

        res
    }
}

