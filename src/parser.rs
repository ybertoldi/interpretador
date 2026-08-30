use std::mem::discriminant;

use crate::{
    ast::{Expr, Program, Stmt},
    scanner::Token::{self, *},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

// main functions
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Program {
        self.program()
    }

    fn program(&mut self) -> Program {
        let mut stmts = Vec::new();
        while !self.is_at_end() {
            let stmt = self.declaration();
            stmts.push(stmt);
        }
        Program(stmts)
    }

    fn declaration(&mut self) -> Stmt {
        if self.check(&Var) {
            self.consume(Var).unwrap();
            self.var_declaration()
        } else {
            self.stmt()
        }
    }

    fn var_declaration(&mut self) -> Stmt {
        let name = self
            .consume(Identifier("".to_string()))
            .expect("No expected identifier after var declaration");

        let initializer = if self.check(&Equal) {
            self.consume(Equal).unwrap();
            Some(self.expression())
        } else {
            None
        };

        self.consume(Semicolon)
            .expect("Expected semicolon after expression");
        Stmt::Var { name, initializer }
    }

    fn stmt(&mut self) -> Stmt {
        let stmt;
        if self.check(&Print) {
            self.consume(Print);
            stmt = Stmt::Print(self.expression());
        } else {
            stmt = Stmt::Expression(self.expression());
        }
        self.consume(Semicolon)
            .expect("Expected ';' after statement");

        stmt
    }

    // parsing
    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();

        while self.matches(&[BangEqual, EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::build_binary(expr, operator, right);
        }
        expr
    }
    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();

        while self.matches(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::build_binary(expr, operator, right);
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr: Expr = self.factor();
        while self.matches(&[Minus, Plus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::build_binary(expr, operator, right);
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary();
        while self.matches(&[Slash, Star]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::build_binary(expr, operator, right);
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.matches(&[Bang, Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::build_unary(operator, right);
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.matches(&[False, True, Nil, Number(0.0), StringLiteral("".to_string())]) {
            return Expr::build_literal(self.previous().clone());
        }

        if self.check(&LeftParen) {
            self.consume(LeftParen).unwrap();

            let expr = self.expression();
            self.consume(Token::RightParen)
                .expect("Expected ')' after expression!");
            return Expr::build_grouping(expr);
        }

        if self.check(&Identifier("".to_string())) {
            return Expr::build_variable(self.advance().clone());
        }

        panic!("Error parsing primary. Found {:?}", self.peek())
    }
}

// utils
impl Parser {
    fn is_at_end(&self) -> bool {
        self.tokens[self.current] == Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            discriminant(self.peek()) == discriminant(token)
        }
    }
    fn matches(&mut self, token_list: &[Token]) -> bool {
        for token in token_list {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    fn consume(&mut self, token: Token) -> Option<Token> {
        if self.check(&token) {
            let token = self.peek().clone();
            if !self.is_at_end() {
                self.current += 1;
            }
            Some(token)
        } else {
            None
        }
    }
}
