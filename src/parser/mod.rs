use ast::*;
use lexer::*;
use token::*;

pub struct Parser {
    pub l: Lexer,
    pub cur_token: Option<Token>,
    pub peek_token: Option<Token>,
}

impl Parser {
    fn new(l: &Lexer) -> Parser {
        let mut p = Parser {l: l.clone(), cur_token: None, peek_token: None};

        p.next_token();
        p.next_token();
        
        p
    }

    fn next_token(&mut self) {
        self.cur_token = match self.peek_token {
            Some(tok) => Some(tok.clone()),
            None => None,
        };
        self.peek_token = Some(self.l.next_token.clone());
    }

    fn parse_program(&mut self) -> Program {
        let program: Program = Program::new();

        loop {
            if self.cur_token.Type != EOF {
                let stmt = self.parse
            } else {
                break;
            }
        }
    }

    fn parse_statement(&self) -> Statement {
        
    }

    fn parse_let_statement(&self) -> LetStatement {
        let stmt = LetStatement
    }
}
