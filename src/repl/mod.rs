use lexer::*;
use std::io;
use token::*;

const PROMPT: &'static str = ">> ";

pub fn start() {
    loop {
        println!("{}", PROMPT);
        let mut scanned = String::new();

        match io::stdin().read_line(&mut scanned) {
            Ok(_) => {
                let mut l = Lexer::new(scanned);

                loop {
                    let mut tok = l.next_token();

                    if tok.t_type != EOF {
                        println!("{:?}", tok);
                    } else {
                        break;
                    }
                }
            }
            Err(_) => break,
        }
    }
}
