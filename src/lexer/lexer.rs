use crate::lexer::token::{Token, TokenType, LiteralValue};

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1
        }
    }

   pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {
       let mut errors = vec![];
       while !self.is_at_end() {
           self.start = self.current;
           match self.scan_token() {
               Ok(_) => (),
               Err(msg) => errors.push(msg)
           }
       }

       self.tokens.push(
           Token {
               token_type: TokenType::Eof,
               lexeme: "".to_string(),
               literal: None,
               line_number: self.line
           }
        );

       if errors.len() > 0 {
           let mut joined_errors = "".to_string();
           errors.iter().for_each(|msg| {
               joined_errors.push_str(&msg);
               joined_errors.push_str("\n");
           });
           return Err(joined_errors);
       }

       Ok(self.tokens.clone())
    }

   fn is_at_end(self: &Self) -> bool {
       self.current >= self.source.len()
   }

   fn scan_token(self: &mut Self) -> Result<(), String> {
       let c = self.advance();

       match c {
           '(' => self.add_token(TokenType::LeftParen),
           ')' => self.add_token(TokenType::RightParen),
           '{' => self.add_token(TokenType::LeftBrace),
           '}' => self.add_token(TokenType::RightBrace),
           ',' => self.add_token(TokenType::Comma),
           '.' => self.add_token(TokenType::Dot),
           '-' => self.add_token(TokenType::Minus),
           '+' => self.add_token(TokenType::Plus),
           ';' => self.add_token(TokenType::SemiColon),
           '*' => self.add_token(TokenType::Star),
           _ => return Err(format!("Unrecognized char at line {}: {}", self.line, c)),
       }
       todo!()
   }

   fn advance(self: &mut Self) -> char {
       let c = self.source.as_bytes()[self.current];
       self.current += 1;

       c as char
   }

   fn add_token(self: &mut Self, token_type: TokenType) {
       self.add_token_literal(token_type, None);
   }

   fn add_token_literal(self: &mut Self, token_type: TokenType, literal: Option<LiteralValue>) {
       let mut text = "".to_string();
       let bytes = self.source.as_bytes();
       for i in self.start .. self.current {
           text.push(bytes[i] as char);
       }
       self.tokens.push(
           Token {
               token_type: token_type,
               lexeme: text,
               literal: literal,
               line_number: self.line
           }
       );
   }
}
