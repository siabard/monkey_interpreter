use token::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Lexer {
    pub input: String,
    pub position: usize,      // current position in input
    pub read_position: usize, // current reading position in input
    pub ch: String,           // current char under examination
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input: input.clone(),
            position: 0,
            read_position: 0,
            ch: String::from(""),
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = String::from("");
        } else {
            self.ch = match self.input.get(self.read_position..(self.read_position + 1)) {
                Some(c) => c.to_string(),
                None => "".to_string(),
            };
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn skip_white_space(&mut self) {
        let mut bytes = self.ch.clone();

        let mut byte = match bytes.get(0..1) {
            Some(b) => b.as_bytes()[0],
            None => 0,
        };

        while byte == 32 || byte == 9 || byte == 10 || byte == 13 {
            self.read_char();

            bytes = self.ch.clone();

            byte = match bytes.get(0..1) {
                Some(b) => b.as_bytes()[0],
                None => 0,
            };
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.read_char();
        self.skip_white_space();
        let tok: Token = match self.ch.as_str() {
            "=" => match self.clone().peek_char().as_str() {
                "=" => {
                    let ch = self.ch.clone();
                    self.read_char();
                    let literal: String = ch + self.ch.clone().as_str();
                    Token::new(EQ, &literal)
                }
                _ => Token::new(ASSIGN, &self.ch),
            },
            "+" => Token::new(PLUS, &self.ch),
            "-" => Token::new(MINUS, &self.ch),
            "!" => match self.clone().peek_char().as_str() {
                "=" => {
                    let ch = self.ch.clone();
                    self.read_char();
                    let literal: String = ch + self.ch.clone().as_str();
                    Token::new(NOT_EQ, &literal)
                }
                _ => Token::new(BANG, &self.ch),
            },

            "/" => Token::new(SLASH, &self.ch),
            "*" => Token::new(ASTERISK, &self.ch),
            "<" => Token::new(LT, &self.ch),
            ">" => Token::new(GT, &self.ch),
            ";" => Token::new(SEMICOLON, &self.ch),
            "(" => Token::new(LPAREN, &self.ch),
            ")" => Token::new(RPAREN, &self.ch),
            "," => Token::new(COMMA, &self.ch),
            "{" => Token::new(LBRACE, &self.ch),
            "}" => Token::new(RBRACE, &self.ch),
            "" => Token::new(EOF, &self.ch),
            _ => if is_letter(&self.ch) {
                let literal = &self.read_idenitifier();
                Token::new(lookup_ident(literal), literal)
            } else if is_digit(&self.ch) {
                let number = &self.read_number();
                Token::new(INT, number)
            } else {
                Token::new(ILLEGAL, &self.ch)
            },
        };
        //println!("{:?}", tok);

        tok
    }

    pub fn read_idenitifier(&mut self) -> String {
        let pos = self.position;

        while is_letter(&self.ch) {
            self.read_char();
        }
        // 포인터가 letter가 아닌 곳까지 이동했다.
        // 포인터를 하나 줄여야한다.

        let identifier = self.input.get(pos..(self.position)).unwrap();

        self.position -= 1;
        self.read_position -= 1;

        identifier.to_string()
    }

    pub fn read_number(&mut self) -> String {
        let pos = self.position;

        while is_digit(&self.ch) {
            self.read_char();
        }
        // 포인터가 number가 아닌 곳까지 이동했다.
        // 포인터를 하나 줄여야한다.

        let number = self.input.get(pos..(self.position)).unwrap();
        self.position -= 1;
        self.read_position -= 1;

        number.to_string()
    }

    pub fn peek_char(self) -> String {
        if self.read_position >= self.input.len() {
            String::from("")
        } else {
            match self.input.get(self.read_position..(self.read_position + 1)) {
                Some(c) => c.to_string(),
                None => "".to_string(),
            }
        }
    }
}

fn is_letter(ch: &String) -> bool {
    let byte = match ch.get(0..1) {
        Some(bytes) => bytes.as_bytes()[0],
        None => 0,
    };
    (byte >= 97 && byte <= 122) || (byte >= 65 && byte <= 90) || (byte == 95)
}

fn is_digit(ch: &String) -> bool {
    let byte = match ch.get(0..1) {
        Some(bytes) => bytes.as_bytes()[0],
        None => 0,
    };
    byte >= 48 && byte <= 57
}
