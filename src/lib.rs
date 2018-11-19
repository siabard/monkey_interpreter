#[macro_use]
extern crate lazy_static;

pub mod lexer;
pub mod repl;
pub mod token;

#[cfg(test)]

mod tests {
    use lexer::*;
    use token::*;

    #[test]
    fn text_next_token() {
        let input = "=+(){},;".to_string();

        let mut l = Lexer::new(input);

        assert_eq!(Token::new(ASSIGN, &"=".to_string()), l.next_token());
        assert_eq!(Token::new(PLUS, &"+".to_string()), l.next_token());
        assert_eq!(Token::new(LPAREN, &"(".to_string()), l.next_token());
        assert_eq!(Token::new(RPAREN, &")".to_string()), l.next_token());
        assert_eq!(Token::new(LBRACE, &"{".to_string()), l.next_token());
        assert_eq!(Token::new(RBRACE, &"}".to_string()), l.next_token());
        assert_eq!(Token::new(COMMA, &",".to_string()), l.next_token());
        assert_eq!(Token::new(SEMICOLON, &";".to_string()), l.next_token());
        assert_eq!(Token::new(EOF, &"".to_string()), l.next_token());
    }

    #[test]
    fn test_next_token2() {
        let input = "
            let five = 5;
            let ten = 10;

            let add = fn(x,y) {
                x + y;
            };

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
  return false;
}

10 == 10;
10 != 9;
        ".to_string();

        let mut l = Lexer::new(input);

        assert_eq!(Token::new(LET, &"let".to_string()), l.next_token());
        assert_eq!(Token::new(IDENT, &"five".to_string()), l.next_token());
        assert_eq!(Token::new(ASSIGN, &"=".to_string()), l.next_token());
        assert_eq!(Token::new(INT, &"5".to_string()), l.next_token());
        assert_eq!(Token::new(SEMICOLON, &";".to_string()), l.next_token());

        assert_eq!(Token::new(LET, &"let".to_string()), l.next_token());
        assert_eq!(Token::new(IDENT, &"ten".to_string()), l.next_token());
        assert_eq!(Token::new(ASSIGN, &"=".to_string()), l.next_token());
        assert_eq!(Token::new(INT, &"10".to_string()), l.next_token());
        assert_eq!(Token::new(SEMICOLON, &";".to_string()), l.next_token());

        assert_eq!(Token::new(LET, &"let".to_string()), l.next_token());
        assert_eq!(Token::new(IDENT, &"add".to_string()), l.next_token());
        assert_eq!(Token::new(ASSIGN, &"=".to_string()), l.next_token());
        assert_eq!(Token::new(FUNCTION, &"fn".to_string()), l.next_token());
        assert_eq!(Token::new(LPAREN, &"(".to_string()), l.next_token());
        assert_eq!(Token::new(IDENT, &"x".to_string()), l.next_token());
        assert_eq!(Token::new(COMMA, &",".to_string()), l.next_token());
        assert_eq!(Token::new(IDENT, &"y".to_string()), l.next_token());
        assert_eq!(Token::new(RPAREN, &")".to_string()), l.next_token());
        assert_eq!(Token::new(LBRACE, &"{".to_string()), l.next_token());

        assert_eq!(Token::new(IDENT, &"x".to_string()), l.next_token());
        assert_eq!(Token::new(PLUS, &"+".to_string()), l.next_token());
        assert_eq!(Token::new(IDENT, &"y".to_string()), l.next_token());
        assert_eq!(Token::new(SEMICOLON, &";".to_string()), l.next_token());
        assert_eq!(Token::new(RBRACE, &"}".to_string()), l.next_token());
        assert_eq!(Token::new(SEMICOLON, &";".to_string()), l.next_token());

        assert_eq!(Token::new(LET, &"let".to_string()), l.next_token());
        assert_eq!(Token::new(IDENT, &"result".to_string()), l.next_token());
        assert_eq!(Token::new(ASSIGN, &"=".to_string()), l.next_token());
        assert_eq!(Token::new(IDENT, &"add".to_string()), l.next_token());
        assert_eq!(Token::new(LPAREN, &"(".to_string()), l.next_token());
        assert_eq!(Token::new(IDENT, &"five".to_string()), l.next_token());
        assert_eq!(Token::new(COMMA, &",".to_string()), l.next_token());
        assert_eq!(Token::new(IDENT, &"ten".to_string()), l.next_token());
        assert_eq!(Token::new(RPAREN, &")".to_string()), l.next_token());
        assert_eq!(Token::new(SEMICOLON, &";".to_string()), l.next_token());

        assert_eq!(Token::new(BANG, &"!".to_string()), l.next_token());
        assert_eq!(Token::new(MINUS, &"-".to_string()), l.next_token());
        assert_eq!(Token::new(SLASH, &"/".to_string()), l.next_token());
        assert_eq!(Token::new(ASTERISK, &"*".to_string()), l.next_token());
        assert_eq!(Token::new(INT, &"5".to_string()), l.next_token());
        assert_eq!(Token::new(SEMICOLON, &";".to_string()), l.next_token());

        assert_eq!(Token::new(INT, &"5".to_string()), l.next_token());
        assert_eq!(Token::new(LT, &"<".to_string()), l.next_token());
        assert_eq!(Token::new(INT, &"10".to_string()), l.next_token());
        assert_eq!(Token::new(GT, &">".to_string()), l.next_token());
        assert_eq!(Token::new(INT, &"5".to_string()), l.next_token());
        assert_eq!(Token::new(SEMICOLON, &";".to_string()), l.next_token());

        assert_eq!(Token::new(IF, &"if".to_string()), l.next_token());
        assert_eq!(Token::new(LPAREN, &"(".to_string()), l.next_token());
        assert_eq!(Token::new(INT, &"5".to_string()), l.next_token());
        assert_eq!(Token::new(LT, &"<".to_string()), l.next_token());
        assert_eq!(Token::new(INT, &"10".to_string()), l.next_token());
        assert_eq!(Token::new(RPAREN, &")".to_string()), l.next_token());
        assert_eq!(Token::new(LBRACE, &"{".to_string()), l.next_token());
        assert_eq!(Token::new(RETURN, &"return".to_string()), l.next_token());
        assert_eq!(Token::new(TRUE, &"true".to_string()), l.next_token());
        assert_eq!(Token::new(SEMICOLON, &";".to_string()), l.next_token());
        assert_eq!(Token::new(RBRACE, &"}".to_string()), l.next_token());
        assert_eq!(Token::new(ELSE, &"else".to_string()), l.next_token());
        assert_eq!(Token::new(LBRACE, &"{".to_string()), l.next_token());
        assert_eq!(Token::new(RETURN, &"return".to_string()), l.next_token());
        assert_eq!(Token::new(FALSE, &"false".to_string()), l.next_token());
        assert_eq!(Token::new(SEMICOLON, &";".to_string()), l.next_token());
        assert_eq!(Token::new(RBRACE, &"}".to_string()), l.next_token());

        assert_eq!(Token::new(INT, &"10".to_string()), l.next_token());
        assert_eq!(Token::new(EQ, &"==".to_string()), l.next_token());
        assert_eq!(Token::new(INT, &"10".to_string()), l.next_token());
        assert_eq!(Token::new(SEMICOLON, &";".to_string()), l.next_token());
        assert_eq!(Token::new(INT, &"10".to_string()), l.next_token());
        assert_eq!(Token::new(NOT_EQ, &"!=".to_string()), l.next_token());
        assert_eq!(Token::new(INT, &"9".to_string()), l.next_token());
        assert_eq!(Token::new(SEMICOLON, &";".to_string()), l.next_token());
        assert_eq!(Token::new(EOF, &"".to_string()), l.next_token());
    }
}
