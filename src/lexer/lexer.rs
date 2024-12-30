use std::collections::HashMap;

use crate::lexer::token::{LiteralValue, Token, TokenType};

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'static str, TokenType>,
}

fn get_keywords() -> HashMap<&'static str, TokenType> {
    HashMap::from([
        ("and", TokenType::And),
        ("class", TokenType::Class),
        ("else", TokenType::Else),
        ("False", TokenType::False),
        ("for", TokenType::For),
        ("fun", TokenType::Function),
        ("if", TokenType::If),
        ("Nil", TokenType::Nil),
        ("or", TokenType::Or),
        ("say", TokenType::Say),
        ("return", TokenType::Return),
        ("super", TokenType::Super),
        ("this", TokenType::This),
        ("True", TokenType::True),
        ("var", TokenType::Variable),
        ("while", TokenType::While),
    ])
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: get_keywords(),
        }
    }

    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {
        let mut errors = vec![];
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(msg) => errors.push(msg),
            }
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line_number: self.line,
        });

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

    fn is_digit(self: &Self, ch: char) -> bool {
        let u_char = ch as u8;
        u_char >= '0' as u8 && u_char <= '9' as u8
    }

    fn is_alphabetical(self: &Self, ch: char) -> bool {
        let u_char = ch as u8;
        (u_char >= 'a' as u8 && u_char <= 'z' as u8)
            || (u_char >= 'A' as u8 && u_char <= 'Z' as u8)
            || (u_char == '_' as u8)
    }

    fn is_alpha_numeric(self: &Self, ch: char) -> bool {
        self.is_alphabetical(ch) || self.is_digit(ch)
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
            '!' => {
                let token = if self.char_match('=') {
                    // !=
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token);
            }
            '=' => {
                let token = if self.char_match('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token);
            }
            '<' => {
                let token = if self.char_match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token);
            }
            '>' => {
                let token = if self.char_match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token);
            }
            '/' => {
                if self.char_match('/') {
                    loop {
                        if self.peek() == '\n' || self.is_at_end() {
                            break;
                        }
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string()?,
            c => {
                if self.is_digit(c) {
                    self.number()?;
                } else if self.is_alphabetical(c) {
                    self.identifier();
                } else {
                    return Err(format!("Unrecognized char at line {}: {}", self.line, c));
                }
            }
        }

        Ok(())
    }

    fn peek(self: &Self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(self: &Self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn identifier(self: &mut Self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let substring = &self.source[self.start..self.current];
        if let Some(&token_type) = self.keywords.get(substring) {
            self.add_token(token_type)
        } else {
            self.add_token(TokenType::Identifier);
        }
    }

    fn number(self: &mut Self) -> Result<(), String> {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        // Look for a decimal point
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance(); // Consume the decimal point

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        let substring = &self.source[self.start..self.current];
        let value = substring.parse::<f64>();
        match value {
            Ok(value) => {
                self.add_token_literal(TokenType::Number, Some(LiteralValue::FValue(value)))
            }
            Err(_) => return Err(format!("Could not parse number: {}", substring)),
        }

        Ok(())
    }

    fn string(self: &mut Self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err("Unterminated string.".to_string());
        }

        self.advance();
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token_literal(
            TokenType::StringLiteral,
            Some(LiteralValue::StringValue(value.to_string())),
        );

        Ok(())
    }

    fn char_match(self: &mut Self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != c {
            return false;
        } else {
            self.current += 1;
            return true;
        }
    }

    fn advance(self: &mut Self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;

        c
    }

    fn add_token(self: &mut Self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(self: &mut Self, token_type: TokenType, literal: Option<LiteralValue>) {
        let text = self.source[self.start..self.current].to_string();

        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text,
            literal: literal,
            line_number: self.line,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_one_char_tokens() {
        let source = "(( )) }{";
        let mut lexer = Lexer::new(source);
        let _ = lexer.scan_tokens();

        assert_eq!(lexer.tokens.len(), 7);
        assert_eq!(lexer.tokens[0].token_type, TokenType::LeftParen);
        assert_eq!(lexer.tokens[1].token_type, TokenType::LeftParen);
        assert_eq!(lexer.tokens[2].token_type, TokenType::RightParen);
        assert_eq!(lexer.tokens[3].token_type, TokenType::RightParen);
        assert_eq!(lexer.tokens[4].token_type, TokenType::RightBrace);
        assert_eq!(lexer.tokens[5].token_type, TokenType::LeftBrace);
        assert_eq!(lexer.tokens[6].token_type, TokenType::Eof);
    }

    #[test]
    fn handle_two_char_tokens() {
        let source = "! != == >= <=";
        let mut lexer = Lexer::new(source);
        let _ = lexer.scan_tokens();

        assert_eq!(lexer.tokens.len(), 6);
        assert_eq!(lexer.tokens[0].token_type, TokenType::Bang);
        assert_eq!(lexer.tokens[1].token_type, TokenType::BangEqual);
        assert_eq!(lexer.tokens[2].token_type, TokenType::EqualEqual);
        assert_eq!(lexer.tokens[3].token_type, TokenType::GreaterEqual);
        assert_eq!(lexer.tokens[4].token_type, TokenType::LessEqual);
        assert_eq!(lexer.tokens[5].token_type, TokenType::Eof);
    }

    #[test]
    fn handle_string_literal() {
        let source = r#""ABC""#;
        let mut lexer = Lexer::new(source);
        let _ = lexer.scan_tokens().unwrap();

        assert_eq!(lexer.tokens.len(), 2);
        assert_eq!(lexer.tokens[0].token_type, TokenType::StringLiteral);
        assert_eq!(lexer.tokens[1].token_type, TokenType::Eof);

        match lexer.tokens[0].literal.as_ref().unwrap() {
            LiteralValue::StringValue(val) => assert_eq!(val, "ABC"),
            _ => panic!("Incorrect literal type"),
        }
    }

    #[test]
    fn handle_unterminated_string() {
        let source = r#""ABC"#;
        let mut lexer = Lexer::new(source);
        let result = lexer.scan_tokens();
        match result {
            Err(_) => (),
            _ => panic!("Should have recognised unterminated string."),
        }
    }

    #[test]
    fn handle_multiline_string() {
        let source = "\"ABC\ndef\"";
        let mut lexer = Lexer::new(source);
        let _ = lexer.scan_tokens().unwrap();

        assert_eq!(lexer.tokens.len(), 2);
        assert_eq!(lexer.tokens[0].token_type, TokenType::StringLiteral);
        match lexer.tokens[0].literal.as_ref().unwrap() {
            LiteralValue::StringValue(val) => assert_eq!(*val, "ABC\ndef"),
            _ => panic!("Incorrect literal type"),
        }
    }

    #[test]
    fn handle_number_literals() {
        let source = "123.123\n321.0\n5";
        let mut lexer = Lexer::new(source);
        lexer.scan_tokens().unwrap();
        assert_eq!(lexer.tokens.len(), 4);

        for i in 0..3 {
            assert_eq!(lexer.tokens[i].token_type, TokenType::Number);
        }

        match lexer.tokens[0].literal.clone().unwrap() {
            LiteralValue::FValue(val) => assert_eq!(val, 123.123),
            _ => panic!("Incorrect literal type"),
        }
        match lexer.tokens[1].literal.clone().unwrap() {
            LiteralValue::FValue(val) => assert_eq!(val, 321.0),
            _ => panic!("Incorrect literal type"),
        }
        match lexer.tokens[2].literal.clone().unwrap() {
            LiteralValue::FValue(val) => assert_eq!(val, 5.0),
            _ => panic!("Incorrect literal type"),
        }
    }

    #[test]
    fn handle_identifier() {
        let source = "this_is_a_variable = 12;";
        let mut lexer = Lexer::new(source);
        lexer.scan_tokens().unwrap();

        assert_eq!(lexer.tokens.len(), 5);

        assert_eq!(lexer.tokens[0].token_type, TokenType::Identifier);
        assert_eq!(lexer.tokens[1].token_type, TokenType::Equal);
        assert_eq!(lexer.tokens[2].token_type, TokenType::Number);
        assert_eq!(lexer.tokens[3].token_type, TokenType::SemiColon);
        assert_eq!(lexer.tokens[4].token_type, TokenType::Eof);
    }

    #[test]
    fn handle_reserved_keywords() {
        let source = "var this_is_a_var = 12;\n while True { say 3};";
        let mut lexer = Lexer::new(source);
        lexer.scan_tokens().unwrap();

        assert_eq!(lexer.tokens.len(), 13);

        assert_eq!(lexer.tokens[0].token_type, TokenType::Variable);
        assert_eq!(lexer.tokens[1].token_type, TokenType::Identifier);
        assert_eq!(lexer.tokens[2].token_type, TokenType::Equal);
        assert_eq!(lexer.tokens[3].token_type, TokenType::Number);
        assert_eq!(lexer.tokens[4].token_type, TokenType::SemiColon);

        assert_eq!(lexer.tokens[5].token_type, TokenType::While);
        assert_eq!(lexer.tokens[6].token_type, TokenType::True);
        assert_eq!(lexer.tokens[7].token_type, TokenType::LeftBrace);
        assert_eq!(lexer.tokens[8].token_type, TokenType::Say);
        assert_eq!(lexer.tokens[9].token_type, TokenType::Number);
        assert_eq!(lexer.tokens[10].token_type, TokenType::RightBrace);
        assert_eq!(lexer.tokens[11].token_type, TokenType::SemiColon);

        assert_eq!(lexer.tokens[12].token_type, TokenType::Eof);
    }
}
