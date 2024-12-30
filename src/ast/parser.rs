use crate::ast::expression::{Expression, LiteralValue};
use crate::lexer::token::{
    Token, TokenType,
    TokenType::{
        Bang, BangEqual, Eof, EqualEqual, Greater, GreaterEqual, LeftParen, Less, LessEqual, Minus,
        Plus, RightParen, Slash, Star,
    },
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

macro_rules! match_tokens {
    ($parser:ident, $($token:ident),+) => {
        {
            let mut result = false;
            {
            $( result |= $parser.match_token($token); )*
            }
            result
        }
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expression {
        self.equality()
    }

    fn comparison(&mut self) -> Expression {
        let mut expression = self.term();

        while self.match_tokens(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous();
            let right = self.term();
            expression = Expression::Binary {
                left: Box::from(expression),
                operator: operator,
                right: Box::from(right),
            }
        }
        expression
    }

    fn term(&mut self) -> Expression {
        let mut expression = self.factor();

        while self.match_tokens(&[Minus, Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expression = Expression::Binary {
                left: Box::from(expression),
                operator: operator,
                right: Box::from(right),
            }
        }

        expression
    }

    fn factor(&mut self) -> Expression {
        let mut expression = self.unary();
        while self.match_tokens(&[Slash, Star]) {
            let operator = self.previous();
            let right = self.unary();
            expression = Expression::Binary {
                left: Box::from(expression),
                operator: operator,
                right: Box::from(right),
            }
        }

        expression
    }

    fn unary(&mut self) -> Expression {
        if self.match_tokens(&[Bang, BangEqual]) {
            let operator = self.previous();
            let right = self.unary();
            Expression::Unary {
                operator: operator,
                right: Box::from(right),
            }
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expression {
        if self.match_token(&LeftParen) {
            let expression = self.expression();
            self.consume(RightParen, "Expected ')'");
            Expression::Grouping {
                expression: Box::from(expression),
            }
        } else {
            let token = self.peek();
            self.advance();
            Expression::Literal {
                value: LiteralValue::from_token(token),
            }
        }
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) {
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
        } else {
            panic!("{}", msg);
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

    fn equality(&mut self) -> Expression {
        let mut expression = self.comparison();

        while self.match_tokens(&[BangEqual, EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expression = Expression::Binary {
                left: Box::from(expression),
                operator,
                right: Box::from(right),
            };
        }

        expression
    }
}
