extern crate monkey_interpreter;
use monkey_interpreter::ast::*;
use monkey_interpreter::lexer::*;
use monkey_interpreter::parser::*;
use monkey_interpreter::token::*;

#[test]
fn test_let_statement() {
    struct TestToken {
        expected_identifier: String,
    };

    let input = "
let x = 5;
let y = 10;
let foobar = 838383;    
    ";

    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(&l);

    if let Some(program) = p.parse_program() {
        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements. got {}",
            program.statements.len()
        );

        let tests = vec![
            TestToken {
                expected_identifier: "x".to_string(),
            },
            TestToken {
                expected_identifier: "y".to_string(),
            },
            TestToken {
                expected_identifier: "foobar".to_string(),
            },
        ];
    } else {
        assert_eq!(true, false, "program might not null");
    }
}
