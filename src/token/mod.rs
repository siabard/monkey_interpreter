type TokenType = &'static str;

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub t_type: TokenType,
    pub literal : String,
}

impl Token {
    pub fn new(token_type: TokenType, ch: &String) -> Token {
        Token {
            t_type : token_type,
            literal: ch.clone(),
        }
    }
}

pub const ILLEGAL:&'static str = "ILLEGAL";
pub const EOF: &'static str = "EOF";

// Identifiers + literals
pub const IDENT: &'static str = "IDENT"; // add, foobar, x, y, ...
pub const INT: &'static str = "INT";

// operator
pub const ASSIGN : &'static str = "=";
pub const PLUS: &'static str = "+";

// Delimiters
pub const COMMA: &'static str = ",";
pub const SEMICOLON : &'static str = ";";

pub const LPAREN: &'static str = "(";
pub const RPAREN : &'static str = ")";
pub const LBRACE : &'static str = "{";
pub const RBRACE : &'static str = "}";

// keywords
pub const FUNCTION: &'static str = "FUNCTION";
pub const LET: &'static str = "LET";