extern crate monkey_interpreter;

use monkey_interpreter::lexer::*;
use monkey_interpreter::token::*;

#[test]
fn text_next_token() {
    let input = "=+(){},;".to_string();

    let mut l = Lexer::new(input);

    assert_eq!(
        Token::new(TokenType::ASSIGN, &"=".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::PLUS, &"+".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::LPAREN, &"(".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::RPAREN, &")".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::LBRACE, &"{".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::RBRACE, &"}".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::COMMA, &",".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::SEMICOLON, &";".to_string()),
        l.next_token()
    );
    assert_eq!(Token::new(TokenType::EOF, &"".to_string()), l.next_token());
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

    assert_eq!(
        Token::new(TokenType::LET, &"let".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::IDENT, &"five".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::ASSIGN, &"=".to_string()),
        l.next_token()
    );
    assert_eq!(Token::new(TokenType::INT, &"5".to_string()), l.next_token());
    assert_eq!(
        Token::new(TokenType::SEMICOLON, &";".to_string()),
        l.next_token()
    );

    assert_eq!(
        Token::new(TokenType::LET, &"let".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::IDENT, &"ten".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::ASSIGN, &"=".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::INT, &"10".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::SEMICOLON, &";".to_string()),
        l.next_token()
    );

    assert_eq!(
        Token::new(TokenType::LET, &"let".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::IDENT, &"add".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::ASSIGN, &"=".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::FUNCTION, &"fn".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::LPAREN, &"(".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::IDENT, &"x".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::COMMA, &",".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::IDENT, &"y".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::RPAREN, &")".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::LBRACE, &"{".to_string()),
        l.next_token()
    );

    assert_eq!(
        Token::new(TokenType::IDENT, &"x".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::PLUS, &"+".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::IDENT, &"y".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::SEMICOLON, &";".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::RBRACE, &"}".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::SEMICOLON, &";".to_string()),
        l.next_token()
    );

    assert_eq!(
        Token::new(TokenType::LET, &"let".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::IDENT, &"result".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::ASSIGN, &"=".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::IDENT, &"add".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::LPAREN, &"(".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::IDENT, &"five".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::COMMA, &",".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::IDENT, &"ten".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::RPAREN, &")".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::SEMICOLON, &";".to_string()),
        l.next_token()
    );

    assert_eq!(
        Token::new(TokenType::BANG, &"!".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::MINUS, &"-".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::SLASH, &"/".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::ASTERISK, &"*".to_string()),
        l.next_token()
    );
    assert_eq!(Token::new(TokenType::INT, &"5".to_string()), l.next_token());
    assert_eq!(
        Token::new(TokenType::SEMICOLON, &";".to_string()),
        l.next_token()
    );

    assert_eq!(Token::new(TokenType::INT, &"5".to_string()), l.next_token());
    assert_eq!(Token::new(TokenType::LT, &"<".to_string()), l.next_token());
    assert_eq!(
        Token::new(TokenType::INT, &"10".to_string()),
        l.next_token()
    );
    assert_eq!(Token::new(TokenType::GT, &">".to_string()), l.next_token());
    assert_eq!(Token::new(TokenType::INT, &"5".to_string()), l.next_token());
    assert_eq!(
        Token::new(TokenType::SEMICOLON, &";".to_string()),
        l.next_token()
    );

    assert_eq!(Token::new(TokenType::IF, &"if".to_string()), l.next_token());
    assert_eq!(
        Token::new(TokenType::LPAREN, &"(".to_string()),
        l.next_token()
    );
    assert_eq!(Token::new(TokenType::INT, &"5".to_string()), l.next_token());
    assert_eq!(Token::new(TokenType::LT, &"<".to_string()), l.next_token());
    assert_eq!(
        Token::new(TokenType::INT, &"10".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::RPAREN, &")".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::LBRACE, &"{".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::RETURN, &"return".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::TRUE, &"true".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::SEMICOLON, &";".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::RBRACE, &"}".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::ELSE, &"else".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::LBRACE, &"{".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::RETURN, &"return".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::FALSE, &"false".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::SEMICOLON, &";".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::RBRACE, &"}".to_string()),
        l.next_token()
    );

    assert_eq!(
        Token::new(TokenType::INT, &"10".to_string()),
        l.next_token()
    );
    assert_eq!(Token::new(TokenType::EQ, &"==".to_string()), l.next_token());
    assert_eq!(
        Token::new(TokenType::INT, &"10".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::SEMICOLON, &";".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::INT, &"10".to_string()),
        l.next_token()
    );
    assert_eq!(
        Token::new(TokenType::NOT_EQ, &"!=".to_string()),
        l.next_token()
    );
    assert_eq!(Token::new(TokenType::INT, &"9".to_string()), l.next_token());
    assert_eq!(
        Token::new(TokenType::SEMICOLON, &";".to_string()),
        l.next_token()
    );
    assert_eq!(Token::new(TokenType::EOF, &"".to_string()), l.next_token());
}
