use crate::yanrlox::token::Token;
use crate::yanrlox::error::Error;
use crate::yanrlox::token::TokenType;

pub struct Scanner {
    stream: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    column: usize

}

impl Scanner {
    pub fn new(context: &str) -> Self {
        Scanner{
            stream: context.chars().collect(), 
            tokens: vec![], 
            start: 0, 
            current: 0, 
            line: 1, 
            column: 1
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Error> {
        
        while self.current <= self.stream.len() {
            self.start = self.current;
            match self.scan_token() {
                Ok(()) => {},
                Err(e) => return Err(e)
            }
        }

        self.push(TokenType::Eof);

        Err(Error::simple_error("This has not been set up yet"))
    }

    pub fn scan_token(&mut self) -> Result<(), Error> {
        let c = self.advance();
        match c {
            '(' => self.push(TokenType::LeftParen),
            ')' => self.push(TokenType::RightParen),
            '{' => self.push(TokenType::LeftBrace),
            '}' => self.push(TokenType::RightBrace),
            ',' => self.push(TokenType::Comma),
            '.' => self.push(TokenType::Dot),
            '-' => self.push(TokenType::Minus),
            '+' => self.push(TokenType::Plus),
            ';' => self.push(TokenType::Semicolon),
            '*' => self.push(TokenType::Star),


            _ => return Err(Error::pos_error(&format!("Unexpected character: |{}|", c), self.line, self.column, "A charater not in the standard definition of lox has be detected"))

        };

        Ok(())
    }

    fn push(&mut self, t: TokenType) {
        self.tokens.push(Token::add_type(t, self.line, self.column, self.start-self.current));
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.column += 1;
        return self.stream[self.current];
      }
}