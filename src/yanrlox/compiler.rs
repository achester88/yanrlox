use std::iter::Scan;

use crate::yanrlox::scanner::Scanner;
use crate::yanrlox::error::*;
use crate::yanrlox::token::TokenType;
use crate::yanrlox::chunk::Chunk;
use crate::yanrlox::token::Token;
use crate::return_err;

use crate::yanrlox::error::*;

use super::{debug::disassembleChunk, scanner, token::{self, TokenValue}};

type ParseFn = fn(&mut Compiler) -> Result<(), Error>;


#[derive(Clone, Copy, Debug)]
enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary
}

#[derive(Clone, Copy, Debug)]
struct ParseRule {
    prefix: ParseFn,
    infix: ParseFn,
    precedence: Precedence,
}

impl ParseRule {
    fn new(prefix: ParseFn, infix: ParseFn, precedence: Precedence) -> Self {
        ParseRule{prefix: prefix, infix: infix, precedence: precedence}
    }
}

#[derive(Debug)]
pub struct Compiler {
    current: Token,
    previous: Token,
    scanner: Scanner,
    rules: [ParseRule; 41], 
    compilingChunk: Chunk,
}

fn prec_to_num(pre: Precedence) -> usize {
    return match pre {
        Precedence::None       => 0,
        Precedence::Assignment => 1,
        Precedence::Or         => 2,
        Precedence::And        => 3,
        Precedence::Equality   => 4,
        Precedence::Comparison => 5,
        Precedence::Term       => 6,
        Precedence::Factor     => 7,
        Precedence::Unary      => 8,
        Precedence::Call       => 9,
        Precedence::Primary    => 10,
    }
}

fn prec_from_num(pre: usize) -> Precedence {
    return match pre {
        1 => Precedence::Assignment,
        2 => Precedence::Or,
        3 => Precedence::And,
        4 => Precedence::Equality,
        5 => Precedence::Comparison,
        6 => Precedence::Term,
        7 => Precedence::Factor,
        8 => Precedence::Unary,
        9 => Precedence::Call,
        10 => Precedence::Primary,

        _ => Precedence::None
    }
}

impl Compiler {

    pub fn new(source: &str) -> Self {

        
        let rules: [ParseRule; 41] = [
        ParseRule::new(Compiler::grouping, Compiler::null, Precedence::None), //LeftParen
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //RightParen
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //LeftBrace
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //RightBrace
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //Comma
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //Dot
        ParseRule::new(Compiler::unary, Compiler::binary, Precedence::Term),     //Minus
        ParseRule::new(Compiler::null, Compiler::binary, Precedence::Term),     //Plus 
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //Semicolon 
        ParseRule::new(Compiler::null, Compiler::binary, Precedence::Factor),     //Slash
        ParseRule::new(Compiler::null, Compiler::binary, Precedence::Factor),     //Star
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //Bang
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //BangEqual
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //Equal
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //EqualEqual
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //Greater
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //GreaterEqual
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //Less
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //LessEqual
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //Identifier
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //String
        ParseRule::new(Compiler::number, Compiler::null, Precedence::None),     //Number
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //And
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //Class
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //Else
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //False
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //Fun
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //For
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //If
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //Nil
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //Or
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //Print
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //Return
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //Super
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //This
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //True 
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //Var
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //While
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //Eof
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //Error
        ParseRule::new(Compiler::null, Compiler::null, Precedence::None),     //Empty
        ];

        Compiler { 
            current: Token::empty(),
            previous: Token::empty(),
            scanner: Scanner::new(source),
            rules: rules,
            compilingChunk: Chunk::new(),
        }
    }

    pub fn compile(&mut self) -> Result<Chunk, Error> {
        return_err!(self.advance());
        return_err!(self.expression());

        return_err!(self.consume(TokenType::Eof, String::from("Expect end of expression")));

        self.end_compiler();

        Ok(self.compilingChunk.clone())
}
    
    fn expression(&mut self) -> Result<(), Error> {
        self.parse_precedence(Precedence::Assignment)
    }
    
    fn parse_precedence(&mut self, level: Precedence) -> Result<(), Error> {
        return_err!(self.advance());

        let prefixrule = self.get_rule(self.previous.token_type).prefix;

        return_err!(prefixrule(self));
        while prec_to_num(level) <= prec_to_num(self.get_rule(self.current.token_type).precedence) {
            return_err!(self.advance());
            let infixrule = self.get_rule(self.previous.token_type).infix;
            return_err!(infixrule(self));
        }

        Ok(())
    }

    fn consume(&mut self, type_of: TokenType, mes: String) -> Result<(), Error> {
        if self.current.token_type != type_of {
           return Err(Error::simple_error(&mes)); 
        }
        self.advance()
    }

    fn end_compiler(&mut self) {
        self.emit_return();

    #[cfg(feature = "debug-print-code")]
    {
        disassembleChunk(&self.compilingChunk);
    }

    }

    fn emit_return(&mut self) {
        self.emit_bytecode(0x00);
    }

    fn advance(&mut self) -> Result<(), Error> {
        self.previous = self.current.clone();

        self.current = match self.scanner.scan_token() {
            Ok(t) => t,
            Err(e) => return Err(e)
        };
    
        Ok(())
    }

    fn get_rule(&mut self, type_of: TokenType) -> ParseRule {
        self.rules[type_of as usize]
    }

    fn emit_bytecode(&mut self, byte: u8) {
        self.compilingChunk.push_u8(byte, self.previous.line);
    }

    fn emit_bytes(&mut self, byte_1: u8, byte_2: u8) {
        self.emit_bytecode(byte_1);
        self.emit_bytecode(byte_2);
    }
   
    fn make_constant(&mut self, val: TokenValue) -> Result<u8, Error> {
        let num_val = match val {
            TokenValue::Number(x) => x,
            _ => 0.0
        };

        let cos = self.compilingChunk.add_constant(num_val);
        if cos > u8::MAX {
            return Err(Error::simple_error("Too many constants in one chunk."))
        }

        Ok(cos)

    }

    fn emit_constant(&mut self, val: TokenValue) -> Result<(), Error>{
        let pos = match self.make_constant(val) {
            Ok(x) => x,
            Err(e) => return Err(e)
        };
        self.emit_bytes(0x01, pos);
        Ok(())
    }

    //****************************************************

    fn null(&mut self) -> Result<(), Error> {
        Err(Error::simple_error("Undefined field while parsing input"))
    }

    fn number(&mut self) -> Result<(), Error> {
        let mut val = self.previous.val.clone().unwrap();
  
        return_err!(self.emit_constant(val));
        Ok(())
    }

    fn grouping(&mut self) -> Result<(), Error> {
        return_err!(self.expression());
        return_err!(self.consume(TokenType::RightParen, String::from("Expected ')' after expression")));
        Ok(())
    }

    fn unary(&mut self) -> Result<(), Error> {
        let op = self.previous.token_type;

        return_err!(self.parse_precedence(Precedence::Unary));

        match op {
            TokenType::Minus => self.emit_bytecode(0x02),
            _ => return Err(Error::simple_error("Unknown Unary Operator Found")),
        }

        Ok(())
    }

    fn binary(&mut self) -> Result<(), Error> {
        let op = self.previous.token_type;
        let rule = self.get_rule(op);

        self.parse_precedence(prec_from_num(prec_to_num(rule.precedence) + 1));
        match op {
            TokenType::Plus => self.emit_bytecode(0x03),
            TokenType::Minus => self.emit_bytecode(0x04),
            TokenType::Star => self.emit_bytecode(0x05),
            TokenType::Slash => self.emit_bytecode(0x06),
            _ => return Err(Error::simple_error("Unknown Binary Operator Found"))
        }

        Ok(())
    }

}
