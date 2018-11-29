use ast::*;
use lexer::*;
use token::*;

pub struct Parser {
    pub l: Lexer,
    pub cur_token: Option<Token>,
    pub peek_token: Option<Token>,
}

impl Parser {
    pub fn new(l: &Lexer) -> Parser {
        let mut p = Parser {
            l: l.clone(),
            cur_token: None,
            peek_token: None,
        };

        p.next_token();
        p.next_token();

        p
    }

    pub fn next_token(&mut self) {
        self.cur_token = match &self.peek_token {
            Some(tok) => Some(tok.clone()),
            None => None,
        };
        self.peek_token = Some(self.l.next_token().clone());
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        None
    }
}
