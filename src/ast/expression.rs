use crate::lexer::token::Token;

pub enum LiteralValue {
    Number(f32),
    StringValue(String),
    True,
    False,
    Nil,
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
            left: Box::from(Unary {
                operator: minus_token,
                right: Box::new(one_two_three),
            }),
            operator: multi,
            right: Box::new(group),
        };
        
        assert_eq!(ast.to_string(), "(* (- 123) (group 45.67))".to_string())
    }
}
