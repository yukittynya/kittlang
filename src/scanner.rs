use std::collections::HashMap;

use crate::tokens::{Token, TokenType, Literal};

pub struct Error {
    pub message: String,
    pub line: usize,
}

pub struct Scanner {
    pub source: Vec<u8>,
    tokens: Vec<Token>,
    start: usize,
    line: usize,
    current: usize,
    err: Option<Error>,
    keywords: HashMap<String, TokenType>
}

impl Scanner {
    pub fn new() -> Self {
        Scanner {
            source: vec![],
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            err: None,
            keywords: vec![
                ("and", TokenType::And),
                ("or", TokenType::Or),
                ("let", TokenType::Let),
                ("mut", TokenType::Mut),
                ("struct", TokenType::Struct),
                ("class", TokenType::Class),
                ("if", TokenType::If),
                ("else", TokenType::Else),
                ("null", TokenType::Null),
                ("true", TokenType::True),
                ("false", TokenType::False),
                ("return", TokenType::Return),
                ("this", TokenType::This),
                ("fn", TokenType::Fn),
                ("while", TokenType::While),
                ("for", TokenType::For),
                ("print", TokenType::Print),
                ("Super", TokenType::Super),
            ]
            .into_iter()
            .map(|(k, v)| (String::from(k), v))
            .collect()
        }
    }
    
    pub fn scan_tokens(&mut self, input: String) {
        self.source = input.into_bytes();

        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }

        match self.err {
            Some(_) => {}
            None => self.tokens.push(Token {
                token_type: TokenType::EOF,
                lexeme: Vec::new(),
                literal: None,
                line: self.line 
            }) 
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;

        char::from(self.source[self.current - 1])
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        
        match c {
            '(' => self.add_token(TokenType::LeftParen),  
            ')' => self.add_token(TokenType::RightParen),  
            '{' => self.add_token(TokenType::LeftBrace),  
            '}' => self.add_token(TokenType::RightBrace),  
            ',' => self.add_token(TokenType::Comma),  
            ';' => self.add_token(TokenType::SemiColon),  
            '.' => self.add_token(TokenType::Dot),  
            '-' => self.add_token(TokenType::Minus),  
            '+' => self.add_token(TokenType::Plus),  
            '*' => self.add_token(TokenType::Star),  
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.at_end() {
                       self.advance(); 
                    } 
                } else {
                    self.add_token(TokenType::Slash);
                }
            } 
            '!' => {
                let check_match = self.match_next('=');

                self.add_token(if check_match {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                });
            }
            '=' => {
                let check_match = self.match_next('=');

                self.add_token(if check_match {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                });
            }
            '<' => {
                let check_match = self.match_next('=');

                self.add_token(if check_match {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                });
            }
            '>' => {
                let check_match = self.match_next('=');

                self.add_token(if check_match {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                });
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {self.err = Some(Error{
                    message: format!("Couldnt scan {}", c),
                    line: self.line
                }) 
                }
            } 
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None); 
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = self.source[self.start .. self.current].to_vec();
        
        self.tokens.push(Token::new(token_type, text, literal, self.line));
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.at_end() {
            return false;
        }

        if char::from(self.source[self.current]) != expected {
            return false;
        }

        self.current += 1;
        true
    }   

    fn at_end(&self) -> bool {
        self.err.is_some() || self.current >= self.source.len()
    }

    fn peek(&self) -> char {
        if self.at_end() {
            return '\0';
        }

        char::from(self.source[self.current])
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        return char::from(self.source[self.current + 1]);
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.advance();
            } 
        }

        if self.at_end() {
            self.err = Some(Error {
                message: "Unterminated string".to_string(),
                line: self.line
            })
        }

        self.advance();

        self.add_token_literal(
            TokenType::String, 
            Some(Literal::String(
                String::from_utf8(self.source[self.start + 1 .. self.current - 1].to_vec()).unwrap()
            ))    
        );
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let value: f64 = String::from_utf8(self.source[self.start .. self.current].to_vec())
            .unwrap()
            .parse()
            .unwrap();

        self.add_token_literal(
            TokenType::Number, 
            Some(Literal::Number(value)) 
        );
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = String::from_utf8(self.source[self.start .. self.current].to_vec()).unwrap();

        let token_type: TokenType = match self.keywords.get(&text) {
            Some(keyword_token_type) => *keyword_token_type,
            None => TokenType::Identifier
        };

        match token_type {
            TokenType::Identifier => {
                self.add_token_literal(
                    TokenType::Identifier, 
                    Some(Literal::Indentifier(text))
                )
            }
            _ => self.add_token(token_type), 
        }
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_' 
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)    
    }

    pub fn print_tokens(&self) {
        for token in &self.tokens {
            println!("{}", token.to_string());
        }
    } 
} 
