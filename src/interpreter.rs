use std::collections::HashMap;

use crate::{
    ast::{Expr, ExpressionVisitor, StatementVisitor, Stmt},
    object::Object,
    scanner::Token,
};
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn run(&mut self, program: &[Stmt]) -> Option<Object> {
        if program.len() < 1 {
            eprintln!("Program is empty");
            return None;
        }

        for stmt in &program[0..program.len() - 1] {
            self.visit_statement(stmt);
        }
        self.visit_statement(program.last().unwrap())
    }
}

impl StatementVisitor<Option<Object>> for Interpreter {
    fn visit_statement(&mut self, stmt: &Stmt) -> Option<Object> {
        match stmt {
            Stmt::Expression(e) => Some(self.eval(e)),
            Stmt::Print(e) => {
                let res = self.eval(e);
                match res {
                    Object::Number(n) => println!("{}", n),
                    Object::Boolean(b) => println!("{}", b),
                    Object::Str(s) => println!("{}", s),
                    Object::Null => println!("(nil)"),
                };
                None
            }
            Stmt::Var { .. } => self.visit_variable_stmt(stmt),
        }
    }

    fn visit_print_stmt(&mut self, _stmt: &Stmt) -> Option<Object> {
        todo!()
    }

    fn visit_expression_stmt(&mut self, _stmt: &Stmt) -> Option<Object> {
        todo!()
    }

    fn visit_variable_stmt(&mut self, stmt: &Stmt) -> Option<Object> {
        if let Stmt::Var { name, initializer } = stmt {
            let value;

            if let Some(expr) = initializer {
                value = Some(self.eval(expr));
            } else {
                value = None
            }

            self.environment.set(&name, value);
        }
        None
    }
}

impl ExpressionVisitor<Object> for Interpreter {
    fn eval(&mut self, expr: &Expr) -> Object {
        match expr {
            Expr::Binary { .. } => self.visit_binary(expr),
            Expr::Unary { .. } => self.visit_unary(expr),
            Expr::Literal(_) => self.visit_literal(expr),
            Expr::Grouping { .. } => self.visit_grouping(expr),
            Expr::Variable { .. } => self.visit_variable_expr(expr),
        }
    }

    fn visit_binary(&mut self, expr: &Expr) -> Object {
        let Expr::Binary {
            left,
            operator,
            right,
        } = expr
        else {
            panic!("Expected binary");
        };

        match operator {
            Token::BangEqual => self.eval(left).nequal(self.eval(right)),
            Token::EqualEqual => self.eval(left).equal(self.eval(right)),

            Token::Greater => self.eval(left).greater(self.eval(right)),
            Token::GreaterEqual => self.eval(left).greater_eq(self.eval(right)),
            Token::Less => self.eval(left).less(self.eval(right)),
            Token::LessEqual => self.eval(left).less_eq(self.eval(right)),

            Token::Plus => self.eval(left).add(self.eval(right)),
            Token::Minus => self.eval(left).sub(self.eval(right)),
            Token::Star => self.eval(left).mult(self.eval(right)),
            Token::Slash => self.eval(left).div(self.eval(right)), // TODO: divisao por zero

            _ => panic!("Invalid operator {:?}", operator),
        }
    }
    fn visit_unary(&mut self, expr: &Expr) -> Object {
        let Expr::Unary { operator, right } = expr else {
            panic!("Expected unary");
        };

        match operator {
            Token::Minus => self.eval(right).negate(),
            Token::Bang => self.eval(right).bang(),
            _ => panic!("Invalid operator {:?}", operator),
        }
    }

    fn visit_literal(&mut self, expr: &Expr) -> Object {
        let Expr::Literal(l) = expr else {
            panic!("expected literal");
        };

        // TODO: remover este clone
        l.clone()
    }

    fn visit_grouping(&mut self, expr: &Expr) -> Object {
        let Expr::Grouping { expression } = expr else {
            panic!("Expected expression");
        };

        self.eval(expression)
    }

    fn visit_variable_expr(&mut self, expr: &Expr) -> Object {
        let Expr::Variable { identifier } = expr else {
            panic!("Expected Variable expression")
        };

        if let Some(val) = self.environment.get(&identifier) {
            return val.clone();
        } else {
            panic!("no value for variable {:?}", identifier);
        }
    }
}

pub struct Environment {
    values: HashMap<String, Option<Object>>,
}

impl Environment {
    fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    fn get(&self, name: &Token) -> Option<&Object> {
        if let Token::Identifier(s) = name {
            match self.values.get(s) {
                Some(v) => v.as_ref(),
                None => panic!(
                    "variavel {} nao registrada no mapa. Mapa = {:?}",
                    s, self.values
                ),
            }
        } else {
            None
        }
    }

    fn set(&mut self, name: &Token, value: Option<Object>) {
        if let Token::Identifier(s) = name {
            self.values.insert(s.clone(), value);
        }
    }
}
