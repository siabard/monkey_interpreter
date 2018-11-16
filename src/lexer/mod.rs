use token::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Lexer {
    pub input : String,
    pub position : usize, // current position in input
    pub read_position: usize, // current reading position in input
    pub ch : String, // current char under examination

}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input: input.clone(),
            position: 0,
            read_position: 0,
            ch : String::from(""),
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = String::from("");
        } else {
            self.ch = self.input.get( self.read_position .. (self.read_position +1)).unwrap().to_string();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.read_char();
        let tok : Token =  match self.ch.as_str() {
            "=" => Token::new(ASSIGN, &self.ch),
            ";" => Token::new(SEMICOLON, &self.ch),
            "(" => Token::new(LPAREN, &self.ch),
            ")" => Token::new(RPAREN, &self.ch),
            "," => Token::new(COMMA, &self.ch),
            "+" => Token::new(PLUS, &self.ch),
            "{" => Token::new(LBRACE, &self.ch),
            "}" => Token::new(RBRACE, &self.ch),
            _ => Token::new(EOF, &"".to_string()),
        };
        tok
    }

}