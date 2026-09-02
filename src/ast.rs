use crate::{object::Object, scanner::Token};

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
    Block(Vec<Stmt>),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
}
impl Stmt {
    pub fn build_if(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> Stmt {
        Self::If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: match else_branch {
                Some(s) => Some(Box::new(s)),
                None => None,
            },
        }
    }
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

    Assignment {
        identifier: Token,
        value: Box<Expr>,
    },

    Literal(Object),
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
            Token::False => Object::Boolean(false),
            Token::True => Object::Boolean(true),
            Token::Number(n) => Object::Number(n),
            Token::Nil => Object::Null,
            Token::StringLiteral(s) => Object::Str(s),
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

    pub fn build_assignment(identifier: Token, value: Expr) -> Self {
        Self::Assignment {
            identifier,
            value: Box::new(value),
        }
    }
}

pub trait StatementVisitor<T> {
    fn visit_statement(&mut self, stmt: &Stmt) -> T {
        match stmt {
            Stmt::Expression(_) => self.visit_expression_stmt(stmt),
            Stmt::Print(_) => self.visit_print_stmt(stmt),
            Stmt::Var { .. } => self.visit_variable_stmt(stmt),
            Stmt::Block(_) => self.visit_block_stmt(stmt),
            Stmt::If { .. } => self.visit_if_stmt(stmt),
        }
    }

    fn visit_print_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_variable_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_block_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_if_stmt(&mut self, stmt: &Stmt) -> T;
}
pub trait ExpressionVisitor<T> {
    fn eval(&mut self, expr: &Expr) -> T {
        match expr {
            Expr::Binary { .. } => self.visit_binary(expr),
            Expr::Unary { .. } => self.visit_unary(expr),
            Expr::Literal(_) => self.visit_literal(expr),
            Expr::Grouping { .. } => self.visit_grouping(expr),
            Expr::Variable { .. } => self.visit_variable_expr(expr),
            Expr::Assignment { .. } => self.visit_assignment_expr(expr),
        }
    }

    fn visit_unary(&mut self, expr: &Expr) -> T;
    fn visit_literal(&mut self, expr: &Expr) -> T;
    fn visit_grouping(&mut self, expr: &Expr) -> T;
    fn visit_binary(&mut self, expr: &Expr) -> T;
    fn visit_variable_expr(&mut self, expr: &Expr) -> T;
    fn visit_assignment_expr(&mut self, expr: &Expr) -> T;
}
