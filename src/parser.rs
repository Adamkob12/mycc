use crate::token::*;
use crate::expr::*;
use crate::stmt::*;

mod expr_parsing;
mod stmt_parsing;

use std::mem::discriminant;

pub struct Parser {
    data: Vec<Lexeme>,
    ptr: usize,
}

impl Parser {
    pub fn new(data: Vec<Lexeme>) -> Self {
        Self {
            data,
            ptr: 0
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, &'static str> {
        let mut ast: Vec<Stmt> = vec![];

        while self.current().tok != Token::EOF {
            ast.push(self.parse_stmt()?);
        }

        for stmt in ast.iter_mut() {
            if let Err(err_position) = stmt.neg_unary_literals() {
                self.change_ptr_to_lexeme(err_position);
                return Err("Cannot Fit Literal into i64")
            }
        }

        Ok(ast)
    }
    
    fn change_ptr_to_lexeme(&mut self, target: Lexeme) {
        self.ptr = 0;

        while self.data[self.ptr] != target {
            self.ptr += 1;
        }
    }

    fn match_tok(&mut self, t: Token) -> bool {
        if self.current().tok == t {
            self.ptr += 1;
            return true
        }

        false
    }

    fn match_tok_type(&mut self, t: Token) -> bool {
        let temp = self.current();

        if discriminant(&temp.tok) == discriminant(&t) {
            self.ptr += 1;
            return true
        }

        false
    }

    pub fn current(&self) -> Lexeme {
        self.data[self.ptr].clone()
    }

    fn previous(&self) -> Lexeme {
        self.data[self.ptr - 1].clone()
    }

    fn look_ahead(&self) -> Lexeme {
        self.data[self.ptr + 1].clone()
    }

    fn go_back(&mut self) {
        self.ptr -= 1;
    }
}