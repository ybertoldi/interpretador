use std::ops::Add;

use crate::scanner::Token;

// TODO: update script for this structure

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
            Token::StringLiteral(s) => LiteralType::StringValue(s), // TODO: passar como referencia
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

pub trait Visitor<T> {
    fn visit_unary(&self, expr: &Expr) -> T;
    fn visit_literal(&self, expr: &Expr) -> T;
    fn visit_grouping(&self, expr: &Expr) -> T;
    fn visit_binary(&self, expr: &Expr) -> T;
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralType {
    Number(f64),
    Boolean(bool),
    StringValue(String), // TODO: make string as reference
    Null,
}

use LiteralType::*;
impl LiteralType {
    pub fn equal(self, rhs: Self) -> Self {
        if self == rhs {
            Boolean(true)
        } else {
            Boolean(false)
        }
    }
    pub fn nequal(self, rhs: Self) -> Self {
        use LiteralType::*;
        if self != rhs {
            Boolean(true)
        } else {
            Boolean(false)
        }
    }
    pub fn add(self, rhs: Self) -> Self {
        match (&self, &rhs) {
            (StringValue(s1), StringValue(s2)) => StringValue(s1.to_string() + s2),
            (Number(n1), Number(n2)) => Number(n1 + n2),
            _ => panic!("can not add {:?} to {:?}", self, rhs),
        }
    }
    pub fn sub(self, rhs: Self) -> Self {
        match (&self, &rhs) {
            (Number(n1), Number(n2)) => Number(n1 - n2),
            _ => panic!("can not sub {:?} to {:?}", self, rhs),
        }
    }
    pub fn div(self, rhs: Self) -> Self {
        match (&self, &rhs) {
            (Number(n1), Number(n2)) => Number(n1 / n2),
            _ => panic!("can not div {:?} to {:?}", self, rhs),
        }
    }
    pub fn mult(self, rhs: Self) -> Self {
        match (&self, &rhs) {
            (StringValue(s1), Number(n)) => StringValue(s1.repeat(*n as usize)),
            (Number(n1), Number(n2)) => Number(n1 * n2),
            _ => panic!("can not multiply {:?} to {:?}", self, rhs),
        }
    }
    pub fn less(self, rhs: Self) -> Self {
        match (&self, &rhs) {
            (Number(n1), Number(n2)) => Boolean(n1 < n2),
            _ => panic!("can not cmp {:?} to {:?}", self, rhs),
        }
    }
    pub fn less_eq(self, rhs: Self) -> Self {
        match (&self, &rhs) {
            (Number(n1), Number(n2)) => Boolean(n1 <= n2),
            _ => panic!("can not cmp {:?} to {:?}", self, rhs),
        }
    }
    pub fn greater(self, rhs: Self) -> Self {
        match (&self, &rhs) {
            (Number(n1), Number(n2)) => Boolean(n1 > n2),
            _ => panic!("can not cmp {:?} to {:?}", self, rhs),
        }
    }
    pub fn greater_eq(self, rhs: Self) -> Self {
        match (&self, &rhs) {
            (Number(n1), Number(n2)) => Boolean(n1 >= n2),
            _ => panic!("can not cmp {:?} to {:?}", self, rhs),
        }
    }

    pub fn bang(self) -> Self {
        match &self {
            Boolean(b) => Boolean(!b),
            _ => panic!("Can not apply bang to {:?}", self),
        }
    }
    pub fn negate(self) -> Self {
        match &self {
            Number(n) => Number(-n),
            _ => panic!("Can not apply unary negation to {:?}", self),
        }
    }
}
