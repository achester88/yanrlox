use crate::yanrlox::scanner::Scanner;
use crate::yanrlox::error::*;
use crate::yanrlox::token::TokenType;

pub fn compile(source: &str) -> Result<(), Error> {
    println!("NOW RUNNING\n\n------\n{}\n-------", source);
    let mut scan = Scanner::new(source);

    let mut line = 0;

    loop {
        let token = match scan.scan_next_token() {
            Ok(t) => t,
            Err(e) => return Err(e)
        };

        if token.line != line {
            print!("{} ", token.line);
            line = token.line;
        } else {
            print!("| ");
        }

        println!("{:?} {:?}", token.token_type, token.val);

        if token.token_type == TokenType::Eof { break }
    }

    /*
    let tokens = match scan.scan_tokens() {
        Ok(val) => val,
        Err(error) => return Err(error)

    };
    */

    //println!("|START|\n{:?}\n|END|", tokens);

    Ok(())
}