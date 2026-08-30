use crate::{literal_type::LiteralType, scanner::Token};

// TODO: update script for this structure

#[derive(Debug)]
pub struct Program(pub Vec<Stmt>);

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
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

    Variable {
        identifier: Token,
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

    pub fn build_variable(identifier: Token) -> Self {
        Self::Variable { identifier }
    }
}

pub trait StatementVisitor<T> {
    fn visit_statement(&mut self, stmt: &Stmt) -> T;
    fn visit_print_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_variable_stmt(&mut self, stmt: &Stmt) -> T;
}
pub trait ExpressionVisitor<T> {
    fn visit_expression(&mut self, expr: &Expr) -> T;
    fn visit_unary(&mut self, expr: &Expr) -> T;
    fn visit_literal(&mut self, expr: &Expr) -> T;
    fn visit_grouping(&mut self, expr: &Expr) -> T;
    fn visit_binary(&mut self, expr: &Expr) -> T;
    fn visit_variable_expr(&mut self, expr: &Expr) -> T;
}
