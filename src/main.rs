use std::io::{stdin, stdout, Write};

use scanner::Scanner;
use token::Token;

mod scanner;
mod token;

fn main() {
    loop {
        print!("> ");
        let _ = stdout().flush();

        let mut input = String::new();

        stdin().read_line(&mut input).expect("Failed to take input");

        let mut scanner = Scanner::new(input);

        let mut count = 0;

        loop {
            let (tok, lit) = scanner.scan();

            if tok == Token::EOF {
                break;
            }

            println!("({:?}, {})", tok, lit);

            count += 1;
        }

        println!("Token Count: {count}");
    }
}
