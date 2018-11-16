pub mod token;
pub mod lexer;

#[cfg(test)]

mod tests {
    #[test]
    fn text_next_token() {
        use token::*;
        use lexer::*;
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
}