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
        // print stmt
        if self.check(&Print) {
            self.consume(Print);
            stmt = Stmt::Print(self.expression());
            self.consume(Semicolon)
                .expect("Expected ';' after statement");
        } else if self.check(&LeftBrace) {
            self.consume(LeftBrace);
            stmt = Stmt::Block(self.block());
        } else if self.check(&If) {
            self.consume(If);
            stmt = self.if_stmt();
        } else if self.matches(&[While]) {
            stmt = self.while_stmt();
        } else {
            stmt = Stmt::Expression(self.expression());
            self.consume(Semicolon)
                .expect("Expected ';' after statement");
        }

        stmt
    }

    fn while_stmt(&mut self) -> Stmt {
        self.consume(LeftParen).expect("Expected '(' after WHILE ");
        let while_cond = self.expression();
        self.consume(RightParen)
            .expect("Expected ')' after WHILE CONDITION");

        let while_stmt = self.stmt();
        Stmt::build_while_stmt(while_cond, while_stmt)
    }

    fn if_stmt(&mut self) -> Stmt {
        self.consume(LeftParen).expect("Expected ( after IF");
        let condition = self.expression();
        self.consume(RightParen)
            .expect("Expected closing parenthesis after if");
        let then_branch = self.stmt();

        let else_branch;
        if self.matches(&[Else]) {
            else_branch = Some(self.stmt());
        } else {
            else_branch = None
        }

        Stmt::build_if(condition, then_branch, else_branch)
    }

    fn block(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while !self.check(&RightBrace) && !self.is_at_end() {
            stmts.push(self.declaration());
        }
        self.consume(RightBrace)
            .expect("Esperava '}' para fechar o bloco");

        stmts
    }

    // parsing
    fn expression(&mut self) -> Expr {
        self.assignment()
    }

    fn assignment(&mut self) -> Expr {
        let expr = self.or();

        if self.matches(&[Token::Equal]) {
            let value = self.assignment();
            let Expr::Variable { identifier } = expr else {
                panic!("invalid assign target");
            };

            Expr::build_assignment(identifier, value)
        } else {
            expr
        }
    }

    fn or(&mut self) -> Expr {
        let mut expr = self.and();

        while self.matches(&[Or]) {
            let operator = self.previous().clone();
            let right = self.and();
            expr = Expr::build_binary(expr, operator, right);
        }

        expr
    }

    fn and(&mut self) -> Expr {
        let mut expr = self.equality();

        while self.matches(&[And]) {
            let operator = self.previous().clone();
            let right = self.equality();
            expr = Expr::build_binary(expr, operator, right);
        }

        expr
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
