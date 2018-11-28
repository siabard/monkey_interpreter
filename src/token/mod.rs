use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQ,
    NOT_EQ,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}
//type TokenType = &'static str;

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub t_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, ch: &String) -> Token {
        Token {
            t_type: token_type,
            literal: ch.clone(),
        }
    }
}

/*
pub const ILLEGAL: &'static str = "ILLEGAL";
pub const EOF: &'static str = "EOF";

// Identifiers + literals
pub const IDENT: &'static str = "IDENT"; // add, foobar, x, y, ...
pub const INT: &'static str = "INT";

// operator
pub const ASSIGN: &'static str = "=";
pub const PLUS: &'static str = "+";
pub const MINUS: &'static str = "-";
pub const BANG: &'static str = "!";
pub const ASTERISK: &'static str = "*";
pub const SLASH: &'static str = "/";
pub const LT: &'static str = "<";
pub const GT: &'static str = ">";
pub const EQ: &'static str = "==";
pub const NOT_EQ: &'static str = "!=";

// Delimiters
pub const COMMA: &'static str = ",";
pub const SEMICOLON: &'static str = ";";

pub const LPAREN: &'static str = "(";
pub const RPAREN: &'static str = ")";
pub const LBRACE: &'static str = "{";
pub const RBRACE: &'static str = "}";

// keywords
pub const FUNCTION: &'static str = "FUNCTION";
pub const LET: &'static str = "LET";
pub const IF: &'static str = "IF";
pub const TRUE: &'static str = "TRUE";
pub const FALSE: &'static str = "FALSE";
pub const ELSE: &'static str = "ELSE";
pub const RETURN: &'static str = "RETURN";
*/

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("fn", TokenType::FUNCTION);
        m.insert("let", TokenType::LET);
        m.insert("true", TokenType::TRUE);
        m.insert("false", TokenType::FALSE);
        m.insert("if", TokenType::IF);
        m.insert("else", TokenType::ELSE);
        m.insert("return", TokenType::RETURN);
        m
    };
}

pub fn lookup_ident(ident: &String) -> TokenType {
    match KEYWORDS.get(ident.as_str()) {
        Some(tok) => tok.clone(),
        None => TokenType::IDENT,
    }
}
