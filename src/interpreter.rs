use crate::{
    ast::{Expr, LiteralType, Visitor},
    scanner::Token,
};

pub struct Interpreter {
    ast: Expr,
}

impl Interpreter {
    pub fn new(ast: Expr) -> Self {
        Self { ast }
    }

    pub fn run(&self) -> LiteralType {
        self.visit(&self.ast)
    }
    fn visit(&self, expr: &Expr) -> LiteralType {
        match expr {
            Expr::Binary { .. } => self.visit_binary(expr),
            Expr::Unary { .. } => self.visit_unary(expr),
            Expr::Literal(v) => v.clone(),
            Expr::Grouping { .. } => self.visit_grouping(expr),
        }
    }
}

impl Visitor<LiteralType> for Interpreter {
    fn visit_binary(&self, expr: &Expr) -> LiteralType {
        let Expr::Binary {
            left,
            operator,
            right,
        } = expr
        else {
            panic!("Expected binary");
        };

        match operator {
            Token::BangEqual => self.visit(left).nequal(self.visit(right)),
            Token::EqualEqual => self.visit(left).equal(self.visit(right)),

            Token::Greater => self.visit(left).greater(self.visit(right)),
            Token::GreaterEqual => self.visit(left).greater_eq(self.visit(right)),
            Token::Less => self.visit(left).less(self.visit(right)),
            Token::LessEqual => self.visit(left).less_eq(self.visit(right)),

            Token::Plus => self.visit(left).add(self.visit(right)),
            Token::Minus => self.visit(left).sub(self.visit(right)),
            Token::Star => self.visit(left).mult(self.visit(right)),
            Token::Slash => self.visit(left).div(self.visit(right)),

            _ => panic!("Invalid operator {:?}", operator),
        }
    }
    fn visit_unary(&self, expr: &Expr) -> LiteralType {
        let Expr::Unary { operator, right } = expr else {
            panic!("Expected unary");
        };

        match operator {
            Token::Minus => self.visit(right).negate(),
            Token::Bang => self.visit(right).bang(),
            _ => panic!("Invalid operator {:?}", operator),
        }
    }

    fn visit_literal(&self, expr: &Expr) -> LiteralType {
        let Expr::Literal(l) = expr else {
            panic!("expected literal");
        };

        // TODO: remover este clone
        l.clone()
    }

    fn visit_grouping(&self, expr: &Expr) -> LiteralType {
        let Expr::Grouping { expression } = expr else {
            panic!("Expected expression");
        };

        self.visit(expr)
    }
}
