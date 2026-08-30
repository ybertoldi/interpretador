use crate::scanner::Token;

// TODO: update script for this structure

#[derive(Debug, Clone)]
pub enum LiteralType {
    Number(f64),
    Boolean(bool),
    String(String), // TODO: make string as reference
    Null,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },

    Literal(LiteralType),
}

impl Expr {
    pub fn build_binary(left: Expr, operator: Token, right: Expr) -> Self {
        Self::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
    pub fn build_grouping(expression: Expr) -> Self {
        Self::Grouping {
            expression: Box::new(expression),
        }
    }
    pub fn build_literal(value: Token) -> Self {
        let value = match value {
            Token::False => LiteralType::Boolean(false),
            Token::True => LiteralType::Boolean(true),
            Token::Number(n) => LiteralType::Number(n),
            Token::Nil => LiteralType::Null,
            _ => {
                panic!("Could not parse literal type for token {:?}", value)
            }
        };

        Self::Literal(value)
    }
    pub fn build_unary(operator: Token, right: Expr) -> Self {
        Self::Unary {
            operator,
            right: Box::new(right),
        }
    }
}
