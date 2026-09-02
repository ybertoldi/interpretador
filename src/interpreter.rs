use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{Expr, ExpressionVisitor, StatementVisitor, Stmt},
    object::Object,
    scanner::Token,
};
pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new())),
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
    fn visit_print_stmt(&mut self, stmt: &Stmt) -> Option<Object> {
        let Stmt::Print(e) = stmt else {
            unreachable!();
        };

        let res = self.eval(e);
        match res {
            Object::Number(n) => println!("{}", n),
            Object::Boolean(b) => println!("{}", b),
            Object::Str(s) => println!("{}", s),
            Object::Null => println!("(nil)"),
        };
        None
    }

    fn visit_variable_stmt(&mut self, stmt: &Stmt) -> Option<Object> {
        if let Stmt::Var { name, initializer } = stmt {
            let value;

            if let Some(expr) = initializer {
                value = Some(self.eval(expr));
            } else {
                value = None
            }

            self.environment.borrow_mut().set(&name, value);
        }
        None
    }

    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> Option<Object> {
        let Stmt::Expression(e) = stmt else {
            unreachable!();
        };

        Some(self.eval(e))
    }

    fn visit_block_stmt(&mut self, stmt: &Stmt) -> Option<Object> {
        let Stmt::Block(stmts) = stmt else {
            unreachable!();
        };

        let prev = Rc::clone(&self.environment);
        let new_ref = Rc::clone(&self.environment);
        self.environment = Rc::new(RefCell::new(Environment::new_with_enclosing(new_ref)));

        for s in stmts {
            self.visit_statement(s);
        }

        self.environment = prev;
        None
    }

    fn visit_if_stmt(&mut self, stmt: &Stmt) -> Option<Object> {
        let Stmt::If {
            condition,
            then_branch,
            else_branch,
        } = stmt
        else {
            panic!("Expected if stmt");
        };

        let enter_if = self.eval(condition).truth_value();
        if enter_if {
            self.visit_statement(then_branch)
        } else if let Some(els) = else_branch {
            self.visit_statement(els)
        } else {
            None
        }
    }
}

impl ExpressionVisitor<Object> for Interpreter {
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

            Token::And => self.eval(left).and(self.eval(right)),
            Token::Or => self.eval(left).or(self.eval(right)),

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

        if let Some(val) = self.environment.borrow().get_token(&identifier) {
            return val;
        } else {
            panic!("no value for variable {:?}", identifier);
        }
    }

    fn visit_assignment_expr(&mut self, expr: &Expr) -> Object {
        let Expr::Assignment { identifier, value } = expr else {
            unreachable!();
        };

        let Token::Identifier(s) = identifier else {
            unreachable!();
        };

        let value = self.eval(value);
        self.environment.borrow_mut().assign(s, value.clone());
        Object::Null
    }
}

pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Option<Object>>,
}

impl Environment {
    fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    fn new_with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }
    }

    fn get_token(&self, name: &Token) -> Option<Object> {
        let Token::Identifier(s) = name else {
            panic!("Token is not identifier!")
        };

        self.get(s)
    }

    fn get(&self, name: &str) -> Option<Object> {
        if let Some(v) = self.values.get(name) {
            return v.clone();
        }

        if let Some(env) = &self.enclosing {
            env.as_ref().borrow().get(name)
        } else {
            None
        }
    }

    fn set(&mut self, name: &Token, value: Option<Object>) {
        if let Token::Identifier(s) = name {
            self.values.insert(s.clone(), value);
        }
    }

    fn assign(&mut self, name: &str, value: Object) {
        if let Some(v) = self.values.get_mut(name) {
            *v = Some(value);
        } else {
            if let Some(env) = &self.enclosing {
                env.borrow_mut().assign(name, value);
            }
        }
    }
}
