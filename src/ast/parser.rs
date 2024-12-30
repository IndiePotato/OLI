use crate::ast::expression::{Expression, LiteralValue};
use crate::lexer::token::{
    Token, TokenType,
    TokenType::{
        Bang, BangEqual, Class, EqualEqual, For, Function, Greater, GreaterEqual, If, LeftParen,
        Less, LessEqual, Minus, Plus, Return, RightParen, Say, SemiColon, Slash, Star, Variable,
        While, True, False, Nil, Number, StringLiteral
    },
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expression, String> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expression, String> {
        self.equality()
    }

    fn comparison(&mut self) -> Result<Expression, String> {
        let mut expression = self.term()?;

        while self.match_tokens(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous();
            let right = self.term()?;
            expression = Expression::Binary {
                left: Box::from(expression),
                operator: operator,
                right: Box::from(right),
            }
        }
        Ok(expression)
    }

    fn term(&mut self) -> Result<Expression, String> {
        let mut expression = self.factor()?;

        while self.match_tokens(&[Minus, Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expression = Expression::Binary {
                left: Box::from(expression),
                operator: operator,
                right: Box::from(right),
            }
        }

        Ok(expression)
    }

    fn factor(&mut self) -> Result<Expression, String> {
        let mut expression = self.unary()?;
        while self.match_tokens(&[Slash, Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expression = Expression::Binary {
                left: Box::from(expression),
                operator: operator,
                right: Box::from(right),
            }
        }

        Ok(expression)
    }

    fn unary(&mut self) -> Result<Expression, String> {
        if self.match_tokens(&[Bang, BangEqual]) {
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Expression::Unary {
                operator: operator,
                right: Box::from(right),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expression, String> {
        let token = self.peek();
        let result;

        match token.token_type {
            LeftParen => {
                self.advance();
                let expression = self.expression()?;
                self.consume(RightParen, "Expected ')'")?;
                result = Expression::Grouping {
                    expression: Box::from(expression),
                }
            }
            False | True | Nil | Number | StringLiteral => {
                self.advance();

                result = Expression::Literal {
                    value: LiteralValue::from_token(token),
                }
            }
            _ => return Err("Expected expression".to_string()),
        }

        Ok(result)
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<(), String> {
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
            Ok(())
        } else {
            Err(msg.to_string())
        }
    }

    fn match_token(&mut self, _type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            if self.peek().token_type == *_type {
                self.advance();
                true
            } else {
                false
            }
        }
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for typ in types {
            if self.match_token(typ) {
                return true;
            }
        }

        false
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn equality(&mut self) -> Result<Expression, String> {
        let mut expression = self.comparison()?;

        while self.match_tokens(&[BangEqual, EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expression = Expression::Binary {
                left: Box::from(expression),
                operator,
                right: Box::from(right),
            };
        }

        Ok(expression)
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == SemiColon {
                return;
            }

            match self.peek().token_type {
                Class | Function | Variable | For | If | While | Say | Return => return,
                _ => (),
            }

            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lexer::Lexer;
    use crate::lexer::token::LiteralValue::IntValue;
    use crate::lexer::token::TokenType::{Number, Plus, SemiColon};

    #[test]
    fn test_addition() {
        let one = Token {
            token_type: Number,
            lexeme: "1".to_string(),
            literal: Some(IntValue(1)),
            line_number: 0,
        };
        let two = Token {
            token_type: Number,
            lexeme: "2".to_string(),
            literal: Some(IntValue(2)),
            line_number: 0,
        };
        let plus = Token {
            token_type: Plus,
            lexeme: "+".to_string(),
            literal: None,
            line_number: 0,
        };
        let semi_colon = Token {
            token_type: SemiColon,
            lexeme: ";".to_string(),
            literal: None,
            line_number: 0,
        };
        let tokens = vec![one, plus, two, semi_colon];
        let mut parser = Parser::new(tokens);
        let parsed_expression = parser.parse().unwrap();
        let string_expression = parsed_expression.to_string();
        assert_eq!(string_expression, "(+ 1 2)");
    }

    #[test]
    fn test_comparison() {
        let source = "1 + 2 == 5 + 7";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let parsed_expression = parser.parse().unwrap();
        let string_expression = parsed_expression.to_string();
        assert_eq!(string_expression, "(== (+ 1 2) (+ 5 7))");
    }

    #[test]
    fn test_eq_with_paren() {
        let source = "1 == (2 + 2);";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let parsed_expression = parser.parse().unwrap();
        let string_expression = parsed_expression.to_string();
        assert_eq!(string_expression, "(== 1 (group (+ 2 2)))");
    }
}
