use std::iter::Scan;

use crate::yanrlox::scanner::Scanner;
use crate::yanrlox::error::*;
use crate::yanrlox::token::TokenType;
use crate::yanrlox::chunk::Chunk;
use crate::yanrlox::token::Token;

use super::{scanner, token};

pub struct Compiler {
    current: Token,
    previous: Token,
    scanner: Scanner
}

impl Compiler {

    pub fn new(source: &str) -> Self {
        Compiler { 
            current: Token::empty(),
            previous: Token::empty(),
            scanner: Scanner::new(source) 
        }
    }

    pub fn compile(&mut self) -> Result<Chunk, Error> {
        let chunk = Chunk::new();
        

        self.advance();
        //self.expression();
        //self.consume(TokenType::Eof, "Expect end of expression");
        

        Ok(chunk)
}

 fn advance(&mut self) -> Result<(), Error> {
    self.previous = self.current.clone();

    self.current = match self.scanner.scan_token() {
        Ok(t) => t,
        Err(e) => return Err(e)
    };

    Ok(())
 }

}