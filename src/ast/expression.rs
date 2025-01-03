use crate::lexer::token::{LiteralValue as TokenLiteralValue, Token, TokenType};

pub enum LiteralValue {
    Number(f32),
    StringValue(String),
    True,
    False,
    Nil,
}

fn unwrap_as_f32(literal: Option<TokenLiteralValue>) -> f32 {
    match literal {
        Some(TokenLiteralValue::IntValue(x)) => x as f32,
        Some(TokenLiteralValue::FValue(x)) => x as f32,
        _ => panic!("Couldn't unwrap as f32"),
    }
}

fn unwrap_as_string(literal: Option<TokenLiteralValue>) -> String {
    match literal {
        Some(TokenLiteralValue::StringValue(s)) => s.clone(),
        Some(TokenLiteralValue::IdentifierValue(s)) => s.clone(),
        _ => panic!("Could not unwrap as string"),
    }
}

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            LiteralValue::Number(x) => x.to_string(),
            LiteralValue::StringValue(x) => x.clone(),
            LiteralValue::True => "True".to_string(),
            LiteralValue::False => "False".to_string(),
            LiteralValue::Nil => "Nil".to_string(),
        }
    }

    pub fn from_token(token: Token) -> Self {
        match token.token_type {
            TokenType::Number => Self::Number(unwrap_as_f32(token.literal)),
            TokenType::StringLiteral => Self::StringValue(unwrap_as_string(token.literal)),
            TokenType::False => Self::False,
            TokenType::True => Self::True,
            TokenType::Nil => Self::Nil,
            _ => panic!("Couldn't create LiteralValue from {:?}", token),
        }
    }
}

pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Grouping {
        expression: Box<Expression>,
    },
    Literal {
        value: LiteralValue,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
}

impl Expression {
    pub fn to_string(&self) -> String {
        match self {
            Expression::Binary {
                left,
                operator,
                right,
            } => format!(
                "({} {} {})",
                operator.lexeme,
                left.to_string(),
                right.to_string()
            ),
            Expression::Grouping { expression } => {
                format!("(group {})", (*expression).to_string())
            }
            Expression::Literal { value } => format!("{}", value.to_string()),
            Expression::Unary { operator, right } => {
                let operator_str = operator.lexeme.clone();
                let right_str = (*right).to_string();
                format!("({} {})", operator_str, right_str)
            }
        }
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::Expression::*;
    use super::LiteralValue::*;
    use super::*;
    use crate::lexer::token::TokenType;

    #[test]
    fn test_pretty_print() {
        let minus_token = Token {
            token_type: TokenType::Minus,
            lexeme: "-".to_string(),
            literal: None,
            line_number: 0,
        };
        let one_two_three = Literal {
            value: Number(123.0),
        };
        let group = Grouping {
            expression: Box::new(Literal {
                value: Number(45.67),
            }),
        };
        let multi = Token {
            token_type: TokenType::Star,
            lexeme: "*".to_string(),
            literal: None,
            line_number: 0,
        };
        let ast = Binary {
            left: Box::new(Unary {
                operator: minus_token,
                right: Box::new(one_two_three),
            }),
            operator: multi,
            right: Box::new(group),
        };

        assert_eq!(ast.to_string(), "(* (- 123) (group 45.67))".to_string())
    }
}
